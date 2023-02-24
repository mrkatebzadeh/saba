mod allocator;
mod config;
mod connection;
mod enforcer;
mod node;
mod scheduler;
mod server;
mod signal;
mod state;
mod topology;

use log::{debug, error, info};
extern crate daemonize;
use crate::state::{
    load_sensitivity_table_from_file, ControllerError, ControllerSettings, ControllerState,
};
use daemonize::Daemonize;
use std::fs::File;
use std::process;
use std::sync::Arc;
use std::thread;

fn main() -> std::io::Result<()> {
    let stdout = File::create("/tmp/saba_controller.out").unwrap();
    let stderr = File::create("/tmp/saba_controller.err").unwrap();
    let pid = "/tmp/saba_controller.pid";

    let config = config::Config::new("/tmp/topology");

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            match config.verbose {
                0 => simplelog::LevelFilter::Warn,
                1 => simplelog::LevelFilter::Info,
                2 => simplelog::LevelFilter::Debug,
                _ => simplelog::LevelFilter::Trace,
            },
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            stderr,
        ),
    ])
    .unwrap();
    debug!("Config: {:?}", config);

    let sensitivity_table_path = match config.sensitivity_table.clone() {
        Some(path) => path,
        None => {
            error!("Sensitivity table path is missing in the configuration");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "sensitivity table path is required",
            ));
        }
    };

    let sensitivity_table = load_sensitivity_table_from_file(&sensitivity_table_path).map_err(
        |err: ControllerError| {
            error!(
                "Failed to load sensitivity table from {}: {}",
                sensitivity_table_path.display(),
                err
            );
            std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
        },
    )?;

    let controller_settings = ControllerSettings {
        queue_budget: config.queue_budget,
        saba_capacity: config.saba_capacity,
        max_priority_levels: config.max_priority_levels,
        min_share: config.min_share,
    };

    let controller_state = Arc::new(
        ControllerState::new(sensitivity_table, controller_settings).map_err(
            |err: ControllerError| {
                error!("Failed to initialize controller state: {}", err);
                std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
            },
        )?,
    );

    let daemonize = Daemonize::new()
        .pid_file(pid) // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .stdout(stdout)
        // .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            let signal_handler = thread::spawn(move || {
                signal::register_exit_signal(pid).unwrap();
            });

            info!("Saba started");

            let server_ip = config.ip.clone();
            let server_port = config.port;
            let server_state = Arc::clone(&controller_state);
            thread::spawn(move || {
                server::serve(&server_ip, server_port, server_state).unwrap();
            });
            signal_handler.join().unwrap();
            process::exit(1);
        }
        Err(e) => error!("Failed to daemonize, {}", e),
    }
    Ok(())
}

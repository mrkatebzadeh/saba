mod config;
mod connection;
mod node;
mod profile;
mod scheduler;
mod server;
mod signal;
mod topology;

use crate::config::Config;
use log::{debug, error, info};
extern crate daemonize;
use daemonize::Daemonize;
use std::fs::File;
use std::process;
use std::thread;

fn main() -> std::io::Result<()> {
    let stdout = File::create("/tmp/saba_controller.out").unwrap();
    let stderr = File::create("/tmp/saba_controller.err").unwrap();
    let pid = "/tmp/saba_controller.pid";

    let config = Config::new("/tmp/topology");

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

            thread::spawn(move || {
                server::serve(&config.ip[..], config.port).unwrap();
            });
            signal_handler.join().unwrap();
            process::exit(1);
        }
        Err(e) => error!("Failed to daemonize, {}", e),
    }
    Ok(())
}

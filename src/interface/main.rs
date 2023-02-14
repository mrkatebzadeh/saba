mod actions;
mod config;
mod signal;
use std::thread;

use crate::config::Config;
use log::{debug, error, info};
extern crate daemonize;
use daemonize::Daemonize;
use std::fs::File;
use std::process;

fn main() -> std::io::Result<()> {
    let stdout = File::create("/tmp/saba_interface.out").unwrap();
    let stderr = File::create("/tmp/saba_interface.err").unwrap();
    let pid = "/tmp/saba_interface.pid";

    let config = Config::new();

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

            {
                let ip = config.ip.clone();
                let port = config.port;
                thread::spawn(move || {
                    actions::init(ip, port).unwrap();
                });
            }
            {
                let ip = config.ip.clone();
                let port = config.port;
                info!("Registering...");
                thread::spawn(move || {
                    actions::register("App1", ip, port, "register").unwrap();
                });
            }
            signal_handler.join().unwrap();
            process::exit(1);
        }
        Err(e) => error!("Failed to daemonize, {}", e),
    }
    Ok(())
}

mod config;
mod connection;
mod node;
mod profile;
mod scheduler;
mod server;
mod topology;

use crate::config::{Config,Commands};
use log::{debug, info};
use std::thread;
use std::fs;

#[tokio::main]
async fn main() {
    let config = Config::new("topology");
    fs::create_dir_all("./logs").expect("Unable to create logs directory");

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
            std::fs::File::create("logs/controller.log").unwrap(),
        ),
    ])
    .unwrap();
    debug!("Config: {:?}", config);

    match config.command {
        Commands::Start => {
            info!("Starting controller...");
            thread::spawn(move || {
                server::serve(config.ip, config.port).unwrap();
            })
            .join()
            .expect("Unable to start server");
        }
        Commands::Stop => {
            info!("Stopping...");
        }
        Commands::Status => {
            info!("Status...");
        }
        Commands::Nop => {}
    }
}

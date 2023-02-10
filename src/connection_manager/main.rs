mod client;
mod register;
mod config;
use std::thread;

use config::{get_config, Commands};
use log::{debug, info};

#[tokio::main]
async fn main() {
    let config = get_config();

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
            std::fs::File::create("connection_manager.log").unwrap(),
        ),
    ])
    .unwrap();
    debug!("Config: {:?}", config);
    match config.command {
        Commands::Start => {
            info!("Starting connection manager...");
            let ip = config.ip.clone();
            let port = config.port;
            let ip2 = config.ip.clone();
            let port2 = config.port;
            thread::spawn(move|| {
                client::connect(ip, port).unwrap();
            })
            .join()
            .expect("Unable to start client");

            info!("Registering...");
            thread::spawn(move|| {
                register::register(ip2, port2).unwrap();
            })
            .join()
            .expect("Unable to register client");
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

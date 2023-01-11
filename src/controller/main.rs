mod server;
mod config;

use std::thread;
use config::{get_config, Commands};
use log::{debug, info, warn, error};

#[tokio::main]
async fn main() {
    let config = get_config();

    simplelog::CombinedLogger::init(
        vec![
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
                std::fs::File::create("controller.log").unwrap(),
            ),
        ]
    ).unwrap();
    debug!("Config: {:?}", config);

    match config.command {
        Commands::Start => {
            info!("Starting controller...");
            thread::spawn(move || {
                server::serve(config.ip, config.port).unwrap();
            }).join().expect("Unable to start server");
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

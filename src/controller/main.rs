mod server;
mod config;

use std::thread;
use config::{get_config, Commands};

#[tokio::main]
async fn main() {
    let config = get_config();
    match config.command {
        Commands::Start => {
            println!("Starting...");
            thread::spawn(move || {
                server::serve(config.ip, config.port).unwrap();
            }).join().expect("Unable to start server");
        }
        Commands::Stop => {
            println!("Stopping...");
        }
        Commands::Status => {
            println!("Status...");
        }
        Commands::Nop => {}
    }
}

extern crate clap;
extern crate serde;
extern crate toml;

use std::thread;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::path::PathBuf;
mod server;


#[derive(Parser)]
#[command(
    about = "Centralized conteroller of Saba bandwidth allocation scheme",
    version = "0.1.0",
    author = "M.R. Siavash Katebzadeh"
)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[arg(short, long)]
    ip: Option<String>,

    #[arg(short, long)]
    port: Option<u16>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Stop,
    Status,
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    controller: Option<ControllerConfig>,
}

#[derive(Debug, Deserialize)]
struct ControllerConfig {
    ip: Option<String>,
    port: Option<u16>,
}

struct Config {
    ip: String,
    port: u16,
}

#[tokio::main]
async fn main() {
    let mut config = Config {
        ip: "127.0.0.1".to_string(),
        port: 8080,
    };

    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
        let toml_config_str =
            std::fs::read_to_string(config_path).expect("Unable to read config file.");
        let toml_config: TomlConfig =
            toml::from_str(&toml_config_str).expect("Unable to parse config file.");
        if let Some(controller_config) = toml_config.controller {
            if let Some(ip) = controller_config.ip {
                config.ip = ip;
            }
            if let Some(port) = controller_config.port {
                config.port = port;
            }
        }
    }

    match cli.verbose {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        _ => println!("Don't be crazy"),
    }

    if let Some(ip) = cli.ip {
        config.ip = ip;
    }

    if let Some(port) = cli.port {
        config.port = port;
    }

    match &cli.command {
        Some(Commands::Start) => {
            println!("Starting...");
            thread::spawn(move || {
                server::serve(config.ip, config.port).unwrap();
            }).join().expect("Unable to start server");
        }
        Some(Commands::Stop) => {
            println!("Stopping...");
        }
        Some(Commands::Status) => {
            println!("Status...");
        }
        None => {}
    }
}

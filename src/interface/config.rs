extern crate clap;
extern crate serde;
extern crate toml;

use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    about = "Interface of Saba bandwidth allocation scheme",
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

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start,
    Stop,
    Status,
    Nop,
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    interface: Option<InterfaceConfig>,
}

#[derive(Debug, Deserialize)]
struct InterfaceConfig {
    ip: Option<String>,
    port: Option<u16>,
}

#[derive(Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub command: Commands,
    pub verbose: u8,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config {
            ip: "127.0.0.1".to_string(),
            port: 8081,
            command: Commands::Nop,
            verbose: 0,
        };

        let cli = Cli::parse();

        if let Some(config_path) = cli.config.as_deref() {
            println!("Value for config: {}", config_path.display());
            let toml_config_str =
                std::fs::read_to_string(config_path).expect("Unable to read config file.");
            let toml_config: TomlConfig =
                toml::from_str(&toml_config_str).expect("Unable to parse config file.");
            if let Some(interface_config) = toml_config.interface {
                if let Some(ip) = interface_config.ip {
                    config.ip = ip;
                }
                if let Some(port) = interface_config.port {
                    config.port = port;
                }
            }
        }

        if Some(cli.verbose).is_some() {
            config.verbose = cli.verbose;
        }

        if let Some(ip) = cli.ip {
            config.ip = ip;
        }

        if let Some(port) = cli.port {
            config.port = port;
        }

        if let Some(command) = cli.command {
            config.command = command;
        }

        config
    }
}

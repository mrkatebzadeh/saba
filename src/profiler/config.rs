extern crate clap;
extern crate serde;
extern crate toml;

use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    about = "Profiler of Saba bandwidth allocation scheme",
    version = "0.1.0",
    author = "M.R. Siavash Katebzadeh"
)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Profile,
    Model,
    AppCluster,
    PriorityCluster,
    Nop,
}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    profiler: Option<ProfilerConfig>,
}

#[derive(Debug, Deserialize)]
struct ProfilerConfig {
    number_of_nodes: Option<u32>,
    degree_of_polynomial: Option<u8>,
}

#[derive(Debug)]
pub struct Config {
    pub application: String,
    pub number_of_nodes: u32,
    pub degree_of_polynomial: u8,
    pub command: Commands,
    pub verbose: u8,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config {
            application: String::new(),
            number_of_nodes: 0,
            degree_of_polynomial: 0,
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
            if let Some(profiler_config) = toml_config.profiler {
                if let Some(number_of_nodes) = profiler_config.number_of_nodes {
                    config.number_of_nodes = number_of_nodes;
                }
                if let Some(degree_of_polynomial) = profiler_config.degree_of_polynomial {
                    config.degree_of_polynomial = degree_of_polynomial;
                }
            }
        }

        if Some(cli.verbose).is_some() {
            config.verbose = cli.verbose;
        }

        if let Some(command) = cli.command {
            config.command = command;
        }

        config
    }
}

/* config.rs

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate clap;
extern crate serde;
extern crate toml;

use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    about = "Centralized conteroller of Saba bandwidth allocation scheme",
    version = "0.1.0",
    author = "M.R. Siavash Katebzadeh"
)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    topology_file: Option<PathBuf>,

    #[arg(long, value_name = "FILE")]
    sensitivity_table: Option<PathBuf>,

    #[arg(long, value_name = "INT")]
    queue_budget: Option<usize>,

    #[arg(long, value_name = "FLOAT")]
    saba_capacity: Option<f32>,

    #[arg(long, value_name = "INT")]
    max_priority_levels: Option<usize>,

    #[arg(long, value_name = "FLOAT")]
    min_share: Option<f32>,

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
    controller: Option<ControllerConfig>,
}

#[derive(Debug, Deserialize)]
struct ControllerConfig {
    ip: Option<String>,
    port: Option<u16>,
    sensitivity_table: Option<PathBuf>,
    queue_budget: Option<usize>,
    saba_capacity: Option<f32>,
    max_priority_levels: Option<usize>,
    min_share: Option<f32>,
}

#[derive(Debug)]
pub struct Config {
    pub topology_file: String,
    pub ip: String,
    pub port: u16,
    pub command: Commands,
    pub verbose: u8,
    pub sensitivity_table: Option<PathBuf>,
    pub queue_budget: usize,
    pub saba_capacity: f32,
    pub max_priority_levels: usize,
    pub min_share: f32,
}

impl Config {
    // Returns the configuration of the controller
    pub fn new(topology_file: &str) -> Config {
        let mut config = Config {
            topology_file: String::from(topology_file),
            ip: "127.0.0.1".to_string(),
            port: 8080,
            command: Commands::Nop,
            verbose: 0,
            sensitivity_table: None,
            queue_budget: 4,
            saba_capacity: 0.9,
            max_priority_levels: 8,
            min_share: 0.0,
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
                if let Some(sensitivity_table) = controller_config.sensitivity_table {
                    config.sensitivity_table = Some(sensitivity_table);
                }
                if let Some(queue_budget) = controller_config.queue_budget {
                    config.queue_budget = queue_budget;
                }
                if let Some(saba_capacity) = controller_config.saba_capacity {
                    config.saba_capacity = saba_capacity;
                }
                if let Some(max_priority_levels) = controller_config.max_priority_levels {
                    config.max_priority_levels = max_priority_levels;
                }
                if let Some(min_share) = controller_config.min_share {
                    config.min_share = min_share;
                }
                if let Some(topology_file) = cli.topology_file.as_deref() {
                    config.topology_file = topology_file.to_string_lossy().to_string();
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

        if let Some(sensitivity_table) = cli.sensitivity_table {
            config.sensitivity_table = Some(sensitivity_table);
        }

        if let Some(queue_budget) = cli.queue_budget {
            config.queue_budget = queue_budget;
        }

        if let Some(saba_capacity) = cli.saba_capacity {
            config.saba_capacity = saba_capacity;
        }

        if let Some(max_priority_levels) = cli.max_priority_levels {
            config.max_priority_levels = max_priority_levels;
        }

        if let Some(min_share) = cli.min_share {
            config.min_share = min_share;
        }

        if let Some(command) = cli.command {
            config.command = command;
        }

        if let Some(topology_file) = cli.topology_file.as_deref() {
            config.topology_file = topology_file.to_string_lossy().to_string();
        }

        config
    }
}

/* config.rs ends here */

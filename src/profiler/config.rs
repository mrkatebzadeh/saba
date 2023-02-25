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
    about = "Profiler of Saba bandwidth allocation scheme",
    version = "0.1.0",
    author = "M.R. Siavash Katebzadeh"
)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[arg(long, value_name = "FILE")]
    profile_csv: Option<PathBuf>,

    #[arg(long, value_name = "FILE")]
    output: Option<PathBuf>,

    #[arg(long, value_name = "INT")]
    degree: Option<usize>,

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
    profile_csv: Option<PathBuf>,
    output: Option<PathBuf>,
    degree_of_polynomial: Option<usize>,
}

#[derive(Debug)]
pub struct Config {
    pub profile_csv: Option<PathBuf>,
    pub output_path: Option<PathBuf>,
    pub degree_of_polynomial: usize,
    pub command: Commands,
    pub verbose: u8,
}

impl Config {
    pub fn new() -> Config {
        let cli = Cli::parse();

        let mut config = Config {
            profile_csv: None,
            output_path: None,
            degree_of_polynomial: 2,
            command: Commands::Nop,
            verbose: cli.verbose,
        };

        if let Some(config_path) = cli.config.as_deref() {
            println!("Value for config: {}", config_path.display());
            let toml_config_str =
                std::fs::read_to_string(config_path).expect("Unable to read config file.");
            let toml_config: TomlConfig =
                toml::from_str(&toml_config_str).expect("Unable to parse config file.");
            if let Some(profiler_config) = toml_config.profiler {
                if let Some(profile_csv) = profiler_config.profile_csv {
                    config.profile_csv = Some(profile_csv);
                }
                if let Some(output) = profiler_config.output {
                    config.output_path = Some(output);
                }
                if let Some(degree_of_polynomial) = profiler_config.degree_of_polynomial {
                    config.degree_of_polynomial = degree_of_polynomial;
                }
            }
        }

        if let Some(profile_csv) = cli.profile_csv {
            config.profile_csv = Some(profile_csv);
        }

        if let Some(output) = cli.output {
            config.output_path = Some(output);
        }

        if let Some(degree) = cli.degree {
            config.degree_of_polynomial = degree;
        }

        if let Some(command) = cli.command {
            config.command = command;
        }

        config
    }
}

/* config.rs ends here */

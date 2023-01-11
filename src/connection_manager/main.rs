extern crate clap;
extern crate toml;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    about = "Connection manager of Saba bandwidth allocation scheme",
    version = "0.1.0",
    author = "M.R. Siavash Katebzadeh"
)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Stop,
    Status,
}
fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli.verbose {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Start) => {
            println!("Starting...");
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


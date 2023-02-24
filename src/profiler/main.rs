mod config;
mod profile;
mod record;
use log::{debug, error, info};
use profile::{Profiler, ProfilerError};
use std::fs::File;
use std::io;

fn main() -> std::io::Result<()> {
    let stderr = File::create("/tmp/saba_profiler.err")?;

    let config = config::Config::new();

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
            stderr,
        ),
    ])
    .unwrap();
    debug!("Config: {:?}", config);

    info!("Saba profiler started");

    if let Err(err) = dispatch(&config) {
        error!("Profiler command failed: {err}");
        return Err(io::Error::other(err.to_string()));
    }

    Ok(())
}

fn dispatch(config: &config::Config) -> Result<(), ProfilerError> {
    match &config.command {
        config::Commands::Profile => run_profile_command(config),
        command => {
            info!("No profiler action requested (command: {:?})", command);
            Ok(())
        }
    }
}

fn run_profile_command(config: &config::Config) -> Result<(), ProfilerError> {
    let profile_csv = config
        .profile_csv
        .as_ref()
        .ok_or(ProfilerError::InvalidConfiguration(
            "profile CSV path is required to run the profile command",
        ))?;
    let output_path = config
        .output_path
        .as_ref()
        .ok_or(ProfilerError::InvalidConfiguration(
            "output path is required to run the profile command",
        ))?;

    let mut profiler = Profiler::new(config.degree_of_polynomial);
    profiler.load_profile_csv(profile_csv)?;
    profiler.build_sensitivity_table()?;
    profiler.write_sensitivity_table(output_path)?;

    let modeled_applications = profiler.sensitivity_table().len();
    info!(
        "Wrote sensitivity table to {} ({} modeled applications)",
        output_path.display(),
        modeled_applications
    );

    Ok(())
}

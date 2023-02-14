use log::info;
use std::fs;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
pub async fn register_exit_signal(pid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = signal(SignalKind::terminate())?;

    info!("Registering SIGTERM.");
    stream.recv().await;
    info!("Received SIGTERM kill signal. Exiting...");
    fs::remove_file(pid)?;

    Ok(())
}

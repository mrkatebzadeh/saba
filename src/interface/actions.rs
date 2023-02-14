use local_ip_address::local_ip;
use log::info;

// ----------------------------------------------
// Init
// ----------------------------------------------

use init::{init_client::InitClient, InitRequest};

pub mod init {
    tonic::include_proto!("init");
}

#[tokio::main]
pub async fn init(ip: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = format!("http://{ip}:{port}").parse()?;
    info!("Connecting to {}", addr);
    let mut client = InitClient::connect(addr).await?;

    let url = local_ip().unwrap().to_string();
    let request = tonic::Request::new(InitRequest { url });
    let response = client.init(request).await?;
    info!("Got: '{}' from service", response.into_inner().confirmation);
    Ok(())
}

use register::{register_client::RegisterClient, RegisterRequest};

// ----------------------------------------------
// Register
// ----------------------------------------------

pub mod register {
    tonic::include_proto!("register");
}

#[tokio::main]
pub async fn register(
    app: &str,
    ip: String,
    port: u16,
    action: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = format!("http://{ip}:{port}").parse()?;
    info!("Registering to {}", addr);
    let mut client = RegisterClient::connect(addr).await?;

    let name = app;
    let request = tonic::Request::new(RegisterRequest {
        name: String::from(name),
        action: String::from(action),
    });
    let response = client.register(request).await?;
    info!("Priority: {}", response.into_inner().priority);
    Ok(())
}

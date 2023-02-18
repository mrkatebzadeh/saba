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

// ----------------------------------------------
// Register
// ----------------------------------------------

use register::{register_client::RegisterClient, RegisterRequest};

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

// ----------------------------------------------
// Connection
// ----------------------------------------------

use connection::{connection_client::ConnectionClient, ConnectionRequest};

pub mod connection {
    tonic::include_proto!("connection");
}

#[tokio::main]
pub async fn connection(
    app: &str,
    ip: String,
    port: u16,
    src: &str,
    dst: &str,
    action: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = format!("http://{ip}:{port}").parse()?;
    info!("Registering connection to {}", addr);
    let mut client = ConnectionClient::connect(addr).await?;

    let name = app;
    let request = tonic::Request::new(ConnectionRequest {
        app: String::from(app),
        src: String::from(src),
        dst: String::from(dst),
        action: String::from(action),
    });
    let response = client.connection(request).await?;
    info!("Res: {}", response.into_inner().res);
    Ok(())
}

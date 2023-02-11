use register::{register_client::RegisterClient, RegisterRequest};
use log::info;


pub mod register {
    tonic::include_proto!("register");
}

#[tokio::main]
pub async fn register(ip: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = format!("http://{ip}:{port}").parse()?;
    info!("Registering to {}", addr);
    let mut client = RegisterClient::connect(addr).await?;

    let name = "Application";
    let request = tonic::Request::new(RegisterRequest {
        name: String::from(name),
    });
    let response = client.register(request).await?;
    info!("Priority: {}", response.into_inner().priority);
    Ok(())
}


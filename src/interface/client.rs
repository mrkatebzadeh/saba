use init::{init_client::InitClient, InitRequest};
use local_ip_address::local_ip;
use log::debug;

pub mod init {
    tonic::include_proto!("init");
}

#[tokio::main]
pub async fn connect(ip: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = format!("http://{ip}:{port}").parse()?;
    debug!("Connecting to {}", addr);
    let mut client = InitClient::connect(addr).await?;

    let url = local_ip().unwrap().to_string();
    let request = tonic::Request::new(InitRequest { url });
    let response = client.init(request).await?;
    println!("Got: '{}' from service", response.into_inner().confirmation);
    Ok(())
}

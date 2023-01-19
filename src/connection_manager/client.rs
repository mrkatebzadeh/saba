use init::{init_client::InitClient, InitRequest};
use log::debug;
use std::io::stdin;
pub mod init {
    tonic::include_proto!("init");
}

#[tokio::main]
pub async fn connect(ip: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = format!("http://{ip}:{port}").parse()?;
    debug!("Connecting to {}", addr);
    let mut client = InitClient::connect(addr).await?;
    loop {
        let mut u = String::new();
        println!("Please provide a url: ");
        stdin().read_line(&mut u).unwrap();
        let u = u.trim();
        let url = match u.trim() {
            "quit" => break,
            _ => u,
        };
        let request = tonic::Request::new(InitRequest {
            url: String::from(url),
        });
        let response = client.init(request).await?;
        println!("Got: '{}' from service", response.into_inner().confirmation);
    }
    Ok(())
}

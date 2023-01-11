use tonic::{transport::Server, Request, Response, Status};
use init::init_server::{Init, InitServer};
use init::{InitRequest, InitResponse};
use log::{debug, info, warn, error};

pub mod init {
    tonic::include_proto!("init");
}

#[derive(Debug,Default)]
pub struct MyInit {}

#[tonic::async_trait]
impl Init for MyInit {
    async fn init(&self, request: Request<InitRequest>) -> Result<Response<InitResponse>, Status> {
        println!("Got a request: {:?}", request);
        let reply = init::InitResponse {
            confirmation: "Hello from the controller!".into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
pub async fn serve(ip: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", ip, port).parse()?;
    let init = MyInit::default();

    info!("Controller listening on {}", addr);

    Server::builder()
        .add_service(InitServer::new(init))
        .serve(addr)
        .await?;

    Ok(())
}



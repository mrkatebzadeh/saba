use init::init_server::{Init, InitServer};
use init::{InitRequest, InitResponse};
use register::register_server::{Register, RegisterServer};
use register::{RegisterRequest, RegisterResponse};
use log::info;
use tonic::{transport::Server, Request, Response, Status};

pub mod init {
    tonic::include_proto!("init");
}

#[derive(Debug, Default)]
pub struct MyInit {}

#[tonic::async_trait]
impl Init for MyInit {
    async fn init(&self, request: Request<InitRequest>) -> Result<Response<InitResponse>, Status> {
        println!("Got a request: {request:?}");
        let reply = init::InitResponse {
            confirmation: "Client is connected to the controller".into(),
        };
        Ok(Response::new(reply))
    }
}

pub mod register {
    tonic::include_proto!("register");
}

#[derive(Debug, Default)]
pub struct MyRegister {}

#[tonic::async_trait]
impl Register for MyRegister {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Got a request: {request:?}");
        let reply = register::RegisterResponse {
            priority: 1,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
pub async fn serve(ip: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{ip}:{port}").parse()?;
    let init = MyInit::default();
    let register = MyRegister::default();

    info!("Controller listening on {}", addr);

    Server::builder()
        .add_service(InitServer::new(init))
        .add_service(RegisterServer::new(register))
        .serve(addr)
        .await?;

    Ok(())
}

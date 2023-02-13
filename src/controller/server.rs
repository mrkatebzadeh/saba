use log::info;
use tonic::{transport::Server, Request, Response, Status};


// ----------------------------------------------
// Init
// ----------------------------------------------

use init::init_server::{Init, InitServer};
use init::{InitRequest, InitResponse};

pub mod init {
    tonic::include_proto!("init");
}

#[derive(Debug, Default)]
pub struct MyInit {}

#[tonic::async_trait]
impl Init for MyInit {
    async fn init(&self, request: Request<InitRequest>) -> Result<Response<InitResponse>, Status> {
        info!("Got a request: {request:?}");
        let reply = init::InitResponse {
            confirmation: "Client is connected to the controller".into(),
        };
        Ok(Response::new(reply))
    }
}

// ----------------------------------------------
// Register
// ----------------------------------------------

use register::register_server::{Register, RegisterServer};
use register::{RegisterRequest, RegisterResponse};

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
        info!("Got a request: {request:?}");
        let reply = register::RegisterResponse {
            priority: 1,
        };
        Ok(Response::new(reply))
    }
}
//
// ----------------------------------------------
// Connection
// ----------------------------------------------

use connection::connection_server::{Connection, ConnectionServer};
use connection::{ConnectionRequest, ConnectionResponse};

pub mod connection {
    tonic::include_proto!("connection");
}

#[derive(Debug, Default)]
pub struct MyConnection {}

#[tonic::async_trait]
impl Connection for MyConnection {
    async fn connection(
        &self,
        request: Request<ConnectionRequest>,
    ) -> Result<Response<ConnectionResponse>, Status> {
        info!("Got a request: {request:?}");
        let reply = connection::ConnectionResponse {
            res: 0,
        };
        Ok(Response::new(reply))
    }
}

// ----------------------------------------------
// Server
// ----------------------------------------------

#[tokio::main]
pub async fn serve(ip: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{ip}:{port}").parse()?;
    let init = MyInit::default();
    let register = MyRegister::default();
    let connection = MyConnection::default();

    info!("Controller listening on {}", addr);

    Server::builder()
        .add_service(InitServer::new(init))
        .add_service(RegisterServer::new(register))
        .add_service(ConnectionServer::new(connection))
        .serve(addr)
        .await?;

    Ok(())
}

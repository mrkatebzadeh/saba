/* server.rs

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use crate::state::{ControllerError, ControllerState};
use log::{info, warn};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};

use connection::connection_server::{Connection, ConnectionServer};
use connection::{ConnectionRequest, ConnectionResponse};
use init::init_server::{Init, InitServer};
use init::{InitRequest, InitResponse};
use register::register_server::{Register, RegisterServer};
use register::{RegisterRequest, RegisterResponse};

pub mod init {
    tonic::include_proto!("init");
}

pub mod register {
    tonic::include_proto!("register");
}

pub mod connection {
    tonic::include_proto!("connection");
}

#[derive(Clone)]
struct ControllerRpcService {
    state: Arc<ControllerState>,
}

impl ControllerRpcService {
    fn new(state: Arc<ControllerState>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl Init for ControllerRpcService {
    async fn init(&self, request: Request<InitRequest>) -> Result<Response<InitResponse>, Status> {
        info!("Init request: {request:?}");
        Ok(Response::new(InitResponse {
            confirmation: "Client is connected to the controller".into(),
        }))
    }
}

#[tonic::async_trait]
impl Register for ControllerRpcService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let request = request.into_inner();
        info!("Register request: {:?}", request);
        let action = request.action.to_lowercase();
        let response = match action.as_str() {
            "register" => {
                let priority = self
                    .state
                    .register_application(&request.name)
                    .map_err(controller_error_to_status)?;
                RegisterResponse {
                    priority: i32::from(priority),
                }
            }
            "deregister" => {
                self.state
                    .deregister_application(&request.name)
                    .map_err(controller_error_to_status)?;
                RegisterResponse { priority: 0 }
            }
            _ => {
                return Err(Status::invalid_argument(
                    "action must be 'register' or 'deregister'",
                ));
            }
        };
        Ok(Response::new(response))
    }
}

#[tonic::async_trait]
impl Connection for ControllerRpcService {
    async fn connection(
        &self,
        request: Request<ConnectionRequest>,
    ) -> Result<Response<ConnectionResponse>, Status> {
        let request = request.into_inner();
        info!("Connection request: {:?}", request);
        let action = request.action.to_lowercase();
        match action.as_str() {
            "create" => self
                .state
                .add_connection(&request.app, &request.src, &request.dst)
                .map_err(controller_error_to_status)?,
            "destroy" => self
                .state
                .remove_connection(&request.app, &request.src, &request.dst)
                .map_err(controller_error_to_status)?,
            _ => {
                return Err(Status::invalid_argument(
                    "action must be 'create' or 'destroy'",
                ));
            }
        }
        Ok(Response::new(ConnectionResponse { res: 0 }))
    }
}

fn controller_error_to_status(err: ControllerError) -> Status {
    match err {
        ControllerError::UnknownApplication(_) => Status::not_found(err.to_string()),
        ControllerError::NotRegistered(_) => Status::failed_precondition(err.to_string()),
        ControllerError::Clustering(_) => Status::resource_exhausted(err.to_string()),
        ControllerError::InvalidSettings(_) => Status::failed_precondition(err.to_string()),
        ControllerError::Io(_) | ControllerError::Serialization(_) => {
            warn!("controller error: {err}");
            Status::internal(err.to_string())
        }
    }
}

#[tokio::main]
pub async fn serve(
    ip: &str,
    port: u16,
    state: Arc<ControllerState>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{ip}:{port}").parse()?;
    let service = ControllerRpcService::new(state);

    info!("Controller listening on {}", addr);

    Server::builder()
        .add_service(InitServer::new(service.clone()))
        .add_service(RegisterServer::new(service.clone()))
        .add_service(ConnectionServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

/* server.rs ends here */

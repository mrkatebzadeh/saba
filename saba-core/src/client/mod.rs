/* mod.rs

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

use async_trait::async_trait;
use local_ip_address::local_ip;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Status};

use crate::proto::connection::connection_client::ConnectionClient;
use crate::proto::connection::ConnectionRequest;
use crate::proto::init::init_client::InitClient;
use crate::proto::init::InitRequest;
use crate::proto::register::register_client::RegisterClient;
use crate::proto::register::RegisterRequest;

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("controller connection failed: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("controller returned error: {0}")]
    Rpc(#[from] Status),
    #[error("failed to determine local IP: {0}")]
    LocalIp(#[from] local_ip_address::Error),
    #[error("client is already registered")]
    AlreadyRegistered,
    #[error("client is not registered")]
    NotRegistered,
    #[error("connection already exists")]
    ConnectionExists,
    #[error("connection not found")]
    UnknownConnection,
    #[error("connections must be closed before deregistering")]
    ConnectionsOpen,
}

#[async_trait]
pub trait ControllerRpc: Send + Sync + Clone + 'static {
    async fn init(&self) -> Result<(), ConnectionError>;
    async fn register(&self, app: &str) -> Result<u8, ConnectionError>;
    async fn deregister(&self, app: &str) -> Result<(), ConnectionError>;
    async fn create_connection(
        &self,
        app: &str,
        src: &str,
        dst: &str,
    ) -> Result<(), ConnectionError>;
    async fn destroy_connection(
        &self,
        app: &str,
        src: &str,
        dst: &str,
    ) -> Result<(), ConnectionError>;
}

#[derive(Clone, Debug)]
pub struct GrpcControllerRpc {
    endpoint: String,
}

impl GrpcControllerRpc {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
        }
    }

    async fn init_client(&self) -> Result<InitClient<tonic::transport::Channel>, ConnectionError> {
        Ok(InitClient::connect(self.endpoint.clone()).await?)
    }

    async fn register_client(
        &self,
    ) -> Result<RegisterClient<tonic::transport::Channel>, ConnectionError> {
        Ok(RegisterClient::connect(self.endpoint.clone()).await?)
    }

    async fn connection_client(
        &self,
    ) -> Result<ConnectionClient<tonic::transport::Channel>, ConnectionError> {
        Ok(ConnectionClient::connect(self.endpoint.clone()).await?)
    }
}

#[async_trait]
impl ControllerRpc for GrpcControllerRpc {
    async fn init(&self) -> Result<(), ConnectionError> {
        let mut client = self.init_client().await?;
        let url = local_ip()?.to_string();
        let req = Request::new(InitRequest { url });
        client.init(req).await?;
        Ok(())
    }

    async fn register(&self, app: &str) -> Result<u8, ConnectionError> {
        let mut client = self.register_client().await?;
        let req = Request::new(RegisterRequest {
            name: app.to_string(),
            action: "register".into(),
        });
        let res = client.register(req).await?.into_inner();
        Ok(res.priority as u8)
    }

    async fn deregister(&self, app: &str) -> Result<(), ConnectionError> {
        let mut client = self.register_client().await?;
        let req = Request::new(RegisterRequest {
            name: app.to_string(),
            action: "deregister".into(),
        });
        client.register(req).await?;
        Ok(())
    }

    async fn create_connection(
        &self,
        app: &str,
        src: &str,
        dst: &str,
    ) -> Result<(), ConnectionError> {
        let mut client = self.connection_client().await?;
        let req = Request::new(ConnectionRequest {
            app: app.to_string(),
            src: src.to_string(),
            dst: dst.to_string(),
            action: "create".into(),
        });
        client.connection(req).await?;
        Ok(())
    }

    async fn destroy_connection(
        &self,
        app: &str,
        src: &str,
        dst: &str,
    ) -> Result<(), ConnectionError> {
        let mut client = self.connection_client().await?;
        let req = Request::new(ConnectionRequest {
            app: app.to_string(),
            src: src.to_string(),
            dst: dst.to_string(),
            action: "destroy".into(),
        });
        client.connection(req).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionHandle {
    key: ConnectionKey,
}

impl ConnectionHandle {
    pub fn src(&self) -> &str {
        &self.key.src
    }

    pub fn dst(&self) -> &str {
        &self.key.dst
    }
}

#[derive(Clone, Debug, Default)]
pub struct ConnectionManager<Rpc = GrpcControllerRpc> {
    app: String,
    rpc: Rpc,
    state: Arc<Mutex<ManagerState>>,
}

#[derive(Debug, Default)]
struct ManagerState {
    initialized: bool,
    registered: bool,
    priority: Option<u8>,
    connections: HashSet<ConnectionKey>,
}

#[derive(Clone, Debug, Eq)]
struct ConnectionKey {
    src: String,
    dst: String,
}

impl ConnectionKey {
    fn new(src: String, dst: String) -> Self {
        Self { src, dst }
    }
}

impl PartialEq for ConnectionKey {
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src && self.dst == other.dst
    }
}

impl Hash for ConnectionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.src.hash(state);
        self.dst.hash(state);
    }
}

impl<Rpc: ControllerRpc> ConnectionManager<Rpc> {
    pub fn with_rpc(app: impl Into<String>, rpc: Rpc) -> Self {
        Self {
            app: app.into(),
            rpc,
            state: Arc::new(Mutex::new(ManagerState::default())),
        }
    }

    pub async fn initialize(&self) -> Result<(), ConnectionError> {
        let needs_init = {
            let state = self.state.lock().await;
            !state.initialized
        };
        if needs_init {
            self.rpc.init().await?;
            let mut state = self.state.lock().await;
            state.initialized = true;
        }
        Ok(())
    }

    pub async fn register(&self) -> Result<u8, ConnectionError> {
        self.initialize().await?;
        let state = self.state.lock().await;
        if state.registered {
            return Err(ConnectionError::AlreadyRegistered);
        }
        drop(state);

        let priority = self.rpc.register(&self.app).await?;
        let mut state = self.state.lock().await;
        state.registered = true;
        state.priority = Some(priority);
        Ok(priority)
    }

    pub async fn deregister(&self) -> Result<(), ConnectionError> {
        let mut state = self.state.lock().await;
        if !state.registered {
            return Err(ConnectionError::NotRegistered);
        }
        if !state.connections.is_empty() {
            return Err(ConnectionError::ConnectionsOpen);
        }
        state.registered = false;
        state.priority = None;
        drop(state);
        self.rpc.deregister(&self.app).await
    }

    pub async fn priority(&self) -> Option<u8> {
        let state = self.state.lock().await;
        state.priority
    }

    pub async fn open_connection(
        &self,
        src: impl Into<String>,
        dst: impl Into<String>,
    ) -> Result<ConnectionHandle, ConnectionError> {
        let key = ConnectionKey::new(src.into(), dst.into());
        {
            let state = self.state.lock().await;
            if !state.registered {
                return Err(ConnectionError::NotRegistered);
            }
        }
        {
            let mut state = self.state.lock().await;
            if !state.connections.insert(key.clone()) {
                return Err(ConnectionError::ConnectionExists);
            }
        }

        if let Err(err) = self
            .rpc
            .create_connection(&self.app, &key.src, &key.dst)
            .await
        {
            let mut state = self.state.lock().await;
            state.connections.remove(&key);
            return Err(err);
        }

        Ok(ConnectionHandle { key })
    }

    pub async fn close_connection(&self, handle: &ConnectionHandle) -> Result<(), ConnectionError> {
        let removed = {
            let mut state = self.state.lock().await;
            state.connections.remove(&handle.key)
        };
        if !removed {
            return Err(ConnectionError::UnknownConnection);
        }
        self.rpc
            .destroy_connection(&self.app, handle.src(), handle.dst())
            .await
    }
}

impl ConnectionManager<GrpcControllerRpc> {
    pub async fn connect(
        app: impl Into<String>,
        controller_addr: impl Into<String>,
    ) -> Result<Self, ConnectionError> {
        let manager = Self::with_rpc(app, GrpcControllerRpc::new(controller_addr));
        manager.initialize().await?;
        Ok(manager)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex as StdMutex;

    #[derive(Clone, Default)]
    struct MockControllerRpc {
        log: Arc<StdMutex<Vec<String>>>,
        priority: u8,
    }

    #[async_trait]
    impl ControllerRpc for MockControllerRpc {
        async fn init(&self) -> Result<(), ConnectionError> {
            self.log("init");
            Ok(())
        }

        async fn register(&self, app: &str) -> Result<u8, ConnectionError> {
            self.log(&format!("register:{app}"));
            Ok(self.priority)
        }

        async fn deregister(&self, app: &str) -> Result<(), ConnectionError> {
            self.log(&format!("deregister:{app}"));
            Ok(())
        }

        async fn create_connection(
            &self,
            app: &str,
            src: &str,
            dst: &str,
        ) -> Result<(), ConnectionError> {
            self.log(&format!("connect:{app}:{src}->{dst}"));
            Ok(())
        }

        async fn destroy_connection(
            &self,
            app: &str,
            src: &str,
            dst: &str,
        ) -> Result<(), ConnectionError> {
            self.log(&format!("disconnect:{app}:{src}->{dst}"));
            Ok(())
        }
    }

    impl MockControllerRpc {
        fn new(priority: u8) -> Self {
            Self {
                log: Arc::new(StdMutex::new(Vec::new())),
                priority,
            }
        }

        fn log(&self, msg: &str) {
            self.log.lock().unwrap().push(msg.to_string());
        }

        fn entries(&self) -> Vec<String> {
            self.log.lock().unwrap().clone()
        }
    }

    #[tokio::test]
    async fn register_and_manage_connections() {
        let rpc = MockControllerRpc::new(7);
        let manager = ConnectionManager::with_rpc("app", rpc.clone());
        manager.initialize().await.unwrap();
        assert_eq!(manager.register().await.unwrap(), 7);
        let handle = manager.open_connection("src-1", "dst-1").await.unwrap();
        manager.close_connection(&handle).await.unwrap();
        manager.deregister().await.unwrap();
        assert!(rpc.entries().contains(&"register:app".into()));
    }

    #[tokio::test]
    async fn prevents_duplicate_registration() {
        let rpc = MockControllerRpc::new(1);
        let manager = ConnectionManager::with_rpc("dup-app", rpc);
        manager.initialize().await.unwrap();
        manager.register().await.unwrap();
        let err = manager.register().await.unwrap_err();
        assert!(matches!(err, ConnectionError::AlreadyRegistered));
    }

    #[tokio::test]
    async fn deregistration_requires_closed_connections() {
        let rpc = MockControllerRpc::new(1);
        let manager = ConnectionManager::with_rpc("app", rpc);
        manager.initialize().await.unwrap();
        manager.register().await.unwrap();
        let _handle = manager.open_connection("node-a", "node-b").await.unwrap();
        let err = manager.deregister().await.unwrap_err();
        assert!(matches!(err, ConnectionError::ConnectionsOpen));
    }

    #[tokio::test]
    async fn prevents_unknown_connection_close() {
        let rpc = MockControllerRpc::new(1);
        let manager = ConnectionManager::with_rpc("app", rpc);
        manager.initialize().await.unwrap();
        let err = manager
            .close_connection(&ConnectionHandle {
                key: ConnectionKey::new("s".into(), "d".into()),
            })
            .await
            .unwrap_err();
        assert!(matches!(err, ConnectionError::UnknownConnection));
    }
}

/* mod.rs ends here */

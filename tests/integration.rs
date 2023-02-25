use async_trait::async_trait;
use saba::client::{ConnectionError, ConnectionManager, ControllerRpc};
use saba::clustering::{cluster_applications, map_priority_levels_to_queues};
use saba::model::{completion_samples_to_slowdown, CompletionSample, Model, SensitivityCurve};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn connection_manager_end_to_end_flow() {
    let backend = Arc::new(TestControllerBackend::new(11));
    let rpc = InMemoryControllerRpc::new(backend.clone());
    let manager = ConnectionManager::with_rpc("analytics", rpc.clone());

    manager.initialize().await.unwrap();
    assert_eq!(manager.register().await.unwrap(), 11);

    let handle = manager.open_connection("node-a", "node-b").await.unwrap();
    manager.close_connection(&handle).await.unwrap();
    manager.deregister().await.unwrap();

    let log = backend.events.lock().unwrap().clone();
    assert!(log.iter().any(|entry| entry == "init"));
    assert!(log.iter().any(|entry| entry == "register:analytics"));
    assert!(log
        .iter()
        .any(|entry| entry == "connect:analytics:node-a->node-b"));
}

#[test]
fn profiling_to_queue_mapping_pipeline() {
    let analytics = build_model(&[(100.0, 10.0), (75.0, 15.0), (50.0, 24.0)]);
    let batch = build_model(&[(100.0, 11.0), (75.0, 13.0), (50.0, 17.0)]);

    let apps = vec![
        ("analytics".to_string(), analytics),
        ("batch".to_string(), batch),
    ];

    let clusters = cluster_applications(&apps, 2, 1).unwrap();
    assert_eq!(clusters.len(), 2);
    let queues = map_priority_levels_to_queues(&clusters, 2).unwrap();
    assert_eq!(queues.len(), 2);
    assert!(queues
        .iter()
        .flat_map(|assignment| assignment.priority_levels.iter())
        .all(|priority| (1..=2).contains(priority)));
}

fn build_model(samples: &[(f32, f32)]) -> Model {
    let completion_samples: Vec<_> = samples
        .iter()
        .map(|(bw, time)| CompletionSample::new(*bw, *time).unwrap())
        .collect();
    let slowdowns = completion_samples_to_slowdown(&completion_samples).unwrap();
    let mut model = Model::SensitivityCurve(SensitivityCurve::new(2));
    model.fit(&slowdowns).unwrap();
    model
}

#[derive(Default)]
struct TestControllerBackend {
    priority: u8,
    registered: Mutex<bool>,
    connections: Mutex<HashSet<(String, String)>>,
    events: Mutex<Vec<String>>,
}

impl TestControllerBackend {
    fn new(priority: u8) -> Self {
        Self {
            priority,
            ..Default::default()
        }
    }
}

#[derive(Clone)]
struct InMemoryControllerRpc {
    backend: Arc<TestControllerBackend>,
}

impl InMemoryControllerRpc {
    fn new(backend: Arc<TestControllerBackend>) -> Self {
        Self { backend }
    }

    fn log(&self, entry: impl Into<String>) {
        self.backend.events.lock().unwrap().push(entry.into());
    }
}

#[async_trait]
impl ControllerRpc for InMemoryControllerRpc {
    async fn init(&self) -> Result<(), ConnectionError> {
        self.log("init");
        Ok(())
    }

    async fn register(&self, app: &str) -> Result<u8, ConnectionError> {
        let mut registered = self.backend.registered.lock().unwrap();
        if *registered {
            return Err(ConnectionError::AlreadyRegistered);
        }
        *registered = true;
        self.log(format!("register:{app}"));
        Ok(self.backend.priority)
    }

    async fn deregister(&self, app: &str) -> Result<(), ConnectionError> {
        let mut registered = self.backend.registered.lock().unwrap();
        if !*registered {
            return Err(ConnectionError::NotRegistered);
        }
        if !self.backend.connections.lock().unwrap().is_empty() {
            return Err(ConnectionError::ConnectionsOpen);
        }
        *registered = false;
        self.log(format!("deregister:{app}"));
        Ok(())
    }

    async fn create_connection(
        &self,
        app: &str,
        src: &str,
        dst: &str,
    ) -> Result<(), ConnectionError> {
        let mut connections = self.backend.connections.lock().unwrap();
        if !*self.backend.registered.lock().unwrap() {
            return Err(ConnectionError::NotRegistered);
        }
        if !connections.insert((src.to_string(), dst.to_string())) {
            return Err(ConnectionError::ConnectionExists);
        }
        self.log(format!("connect:{app}:{src}->{dst}"));
        Ok(())
    }

    async fn destroy_connection(
        &self,
        app: &str,
        src: &str,
        dst: &str,
    ) -> Result<(), ConnectionError> {
        let mut connections = self.backend.connections.lock().unwrap();
        if !connections.remove(&(src.to_string(), dst.to_string())) {
            return Err(ConnectionError::UnknownConnection);
        }
        self.log(format!("disconnect:{app}:{src}->{dst}"));
        Ok(())
    }
}

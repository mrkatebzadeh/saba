use crate::allocator::{AppAllocation, SabaAllocator};
use crate::connection::Connection;
use crate::enforcer::{EnforcementPlan, Enforcer, MockSwitchEnforcer};
use log::debug;
use saba::clustering::{
    cluster_applications, map_priority_levels_to_queues, ClusteringError, QueueAssignment,
};
use saba::model::{Model, SensitivityCurve, SensitivityScore};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ControllerSettings {
    pub queue_budget: usize,
    pub saba_capacity: f32,
    pub max_priority_levels: usize,
    pub min_share: f32,
}

#[derive(Debug)]
pub enum ControllerError {
    Io(std::io::Error),
    Serialization(serde_json::Error),
    UnknownApplication(String),
    NotRegistered(String),
    Clustering(ClusteringError),
    InvalidSettings(&'static str),
}

impl fmt::Display for ControllerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ControllerError::Io(err) => write!(f, "I/O error: {err}"),
            ControllerError::Serialization(err) => write!(f, "serialization error: {err}"),
            ControllerError::UnknownApplication(app) => {
                write!(
                    f,
                    "application '{app}' is not present in the sensitivity table"
                )
            }
            ControllerError::NotRegistered(app) => {
                write!(
                    f,
                    "application '{app}' is not registered with the controller"
                )
            }
            ControllerError::Clustering(err) => write!(f, "clustering error: {err}"),
            ControllerError::InvalidSettings(msg) => {
                write!(f, "invalid controller settings: {msg}")
            }
        }
    }
}

impl std::error::Error for ControllerError {}

impl From<std::io::Error> for ControllerError {
    fn from(value: std::io::Error) -> Self {
        ControllerError::Io(value)
    }
}

impl From<serde_json::Error> for ControllerError {
    fn from(value: serde_json::Error) -> Self {
        ControllerError::Serialization(value)
    }
}

impl From<ClusteringError> for ControllerError {
    fn from(value: ClusteringError) -> Self {
        ControllerError::Clustering(value)
    }
}

#[derive(Debug, Clone)]
struct RegisteredApplication {
    name: String,
    priority_level: u8,
    model: Model,
}

#[derive(Debug)]
pub struct ControllerState {
    sensitivity_table: HashMap<String, Model>,
    settings: ControllerSettings,
    registered: Mutex<HashMap<String, RegisteredApplication>>,
    connections: Mutex<HashMap<Connection, String>>,
    allocator: SabaAllocator,
    enforcer: Mutex<Box<dyn Enforcer + Send>>, // boxed to allow swapping in tests
    queue_assignments: Mutex<Vec<QueueAssignment>>,
    app_allocations: Mutex<Vec<AppAllocation>>,
}

impl ControllerState {
    pub fn new(
        sensitivity_table: HashMap<String, Model>,
        settings: ControllerSettings,
    ) -> Result<Self, ControllerError> {
        if settings.queue_budget == 0 {
            return Err(ControllerError::InvalidSettings(
                "queue budget must be greater than zero",
            ));
        }
        if settings.max_priority_levels == 0 {
            return Err(ControllerError::InvalidSettings(
                "max priority levels must be greater than zero",
            ));
        }
        if !(0.0..=1.0).contains(&settings.saba_capacity) {
            return Err(ControllerError::InvalidSettings(
                "Saba capacity must be between 0 and 1",
            ));
        }
        if settings.min_share < 0.0 {
            return Err(ControllerError::InvalidSettings(
                "minimum share cannot be negative",
            ));
        }

        Ok(Self {
            sensitivity_table,
            settings: settings.clone(),
            registered: Mutex::new(HashMap::new()),
            connections: Mutex::new(HashMap::new()),
            allocator: SabaAllocator::new(200, 0.05, settings.min_share),
            enforcer: Mutex::new(
                Box::new(MockSwitchEnforcer::default()) as Box<dyn Enforcer + Send>
            ),
            queue_assignments: Mutex::new(Vec::new()),
            app_allocations: Mutex::new(Vec::new()),
        })
    }

    pub fn register_application(&self, name: &str) -> Result<u8, ControllerError> {
        let model = self
            .sensitivity_table
            .get(name)
            .cloned()
            .ok_or_else(|| ControllerError::UnknownApplication(name.to_string()))?;
        {
            let mut registered = self.registered.lock().unwrap();
            registered
                .entry(name.to_string())
                .and_modify(|app| app.model = model.clone())
                .or_insert(RegisteredApplication {
                    name: name.to_string(),
                    priority_level: 0,
                    model,
                });
        }
        self.recompute_plan()?;
        let registered = self.registered.lock().unwrap();
        Ok(registered
            .get(name)
            .map(|app| app.priority_level)
            .unwrap_or(0))
    }

    pub fn deregister_application(&self, name: &str) -> Result<(), ControllerError> {
        let removed = self.registered.lock().unwrap().remove(name);
        if removed.is_none() {
            return Err(ControllerError::NotRegistered(name.to_string()));
        }
        self.connections
            .lock()
            .unwrap()
            .retain(|_, owner| owner != name);
        self.recompute_plan()?;
        Ok(())
    }

    pub fn add_connection(&self, app: &str, src: &str, dst: &str) -> Result<(), ControllerError> {
        if !self.registered.lock().unwrap().contains_key(app) {
            return Err(ControllerError::NotRegistered(app.to_string()));
        }
        let connection = Connection::new(src.to_string(), dst.to_string());
        self.connections
            .lock()
            .unwrap()
            .insert(connection, app.to_string());
        self.recompute_plan()?;
        Ok(())
    }

    pub fn remove_connection(
        &self,
        app: &str,
        src: &str,
        dst: &str,
    ) -> Result<(), ControllerError> {
        let connection = Connection::new(src.to_string(), dst.to_string());
        let removed = self.connections.lock().unwrap().remove(&connection);
        if removed.is_none() {
            return Err(ControllerError::UnknownApplication(app.to_string()));
        }
        self.recompute_plan()?;
        Ok(())
    }

    pub fn last_plan(&self) -> EnforcementPlan {
        EnforcementPlan {
            queue_assignments: self.queue_assignments.lock().unwrap().clone(),
            app_weights: self.app_allocations.lock().unwrap().clone(),
        }
    }

    pub fn set_enforcer(&self, enforcer: Box<dyn Enforcer + Send>) {
        *self.enforcer.lock().unwrap() = enforcer;
    }

    fn recompute_plan(&self) -> Result<(), ControllerError> {
        let applications: Vec<(String, Model)> = {
            let registered = self.registered.lock().unwrap();
            if registered.is_empty() {
                self.queue_assignments.lock().unwrap().clear();
                self.app_allocations.lock().unwrap().clear();
                self.enforcer.lock().unwrap().enforce(&EnforcementPlan {
                    queue_assignments: Vec::new(),
                    app_weights: Vec::new(),
                });
                return Ok(());
            }
            registered
                .iter()
                .map(|(name, app)| (name.clone(), app.model.clone()))
                .collect()
        };

        let clusters = cluster_applications(&applications, self.settings.max_priority_levels, 0)?;
        let queue_assignments =
            map_priority_levels_to_queues(&clusters, self.settings.queue_budget)?;
        {
            let mut registered = self.registered.lock().unwrap();
            for cluster in &clusters {
                for app in &cluster.applications {
                    if let Some(record) = registered.get_mut(app) {
                        record.priority_level = cluster.priority_level;
                    }
                }
            }
        }

        let app_allocations = self
            .allocator
            .allocate(&applications, self.settings.saba_capacity);

        {
            let mut stored_assignments = self.queue_assignments.lock().unwrap();
            *stored_assignments = queue_assignments.clone();
        }
        {
            let mut stored_allocations = self.app_allocations.lock().unwrap();
            *stored_allocations = app_allocations.clone();
        }

        let plan = EnforcementPlan {
            queue_assignments,
            app_weights: app_allocations,
        };
        self.enforcer.lock().unwrap().enforce(&plan);
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct PersistedModel {
    app: String,
    #[serde(rename = "type")]
    model_type: String,
    degree: Option<usize>,
    coefficients: Vec<f32>,
}

pub fn load_sensitivity_table_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<String, Model>, ControllerError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let models: Vec<PersistedModel> = serde_json::from_reader(reader)?;
    let mut table = HashMap::new();
    for entry in models {
        let model = match entry.model_type.as_str() {
            "curve" => {
                let degree = entry
                    .degree
                    .unwrap_or_else(|| entry.coefficients.len().saturating_sub(1));
                Model::SensitivityCurve(SensitivityCurve {
                    coefficients: entry.coefficients.clone(),
                    degree_of_polynomial: degree,
                })
            }
            "score" => Model::SensitivityScore(SensitivityScore {
                score: entry.coefficients.get(0).copied().unwrap_or(1.0),
            }),
            _ => {
                debug!("Skipping unsupported model type: {}", entry.model_type);
                continue;
            }
        };
        table.insert(entry.app.clone(), model);
    }
    Ok(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use saba::model::SensitivityCurve;
    use tempfile::NamedTempFile;

    fn curve_model(coefficients: Vec<f32>) -> Model {
        Model::SensitivityCurve(SensitivityCurve {
            degree_of_polynomial: coefficients.len() - 1,
            coefficients,
        })
    }

    fn score_model(value: f32) -> Model {
        Model::SensitivityScore(SensitivityScore { score: value })
    }

    fn test_settings() -> ControllerSettings {
        ControllerSettings {
            queue_budget: 2,
            saba_capacity: 1.0,
            max_priority_levels: 2,
            min_share: 0.0,
        }
    }

    #[test]
    fn registers_applications_and_assigns_priorities() {
        let models = HashMap::from([
            ("app_a".to_string(), curve_model(vec![5.0, -5.0])),
            ("app_b".to_string(), score_model(1.0)),
        ]);
        let state = ControllerState::new(models, test_settings()).unwrap();
        let priority = state.register_application("app_a").unwrap();
        assert_eq!(priority, 0);
        assert!(state.register_application("app_b").is_ok());
        let plan = state.last_plan();
        assert_eq!(plan.app_weights.len(), 2);
    }

    #[test]
    fn loads_sensitivity_table_from_json() {
        let file = NamedTempFile::new().unwrap();
        std::fs::write(
            &file,
            r#"[
                {"app": "app_a", "type": "score", "coefficients": [1.0]},
                {"app": "app_b", "type": "curve", "degree": 1, "coefficients": [1.0, -1.0]}
            ]"#,
        )
        .unwrap();
        let table = load_sensitivity_table_from_file(file.path()).unwrap();
        assert_eq!(table.len(), 2);
    }
}

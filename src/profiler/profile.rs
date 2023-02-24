use crate::record::Record;
use csv::ReaderBuilder;
use log::debug;
use saba::model::{
    completion_samples_to_slowdown, CompletionSample, Model, ModelError, SensitivityCurve,
    SensitivityScore,
};
use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Condvar, Mutex};

#[derive(Debug)]
pub enum ProfilerError {
    Io(std::io::Error),
    Csv(csv::Error),
    Model(ModelError),
    MissingProfiles,
    Serialization(serde_json::Error),
    InvalidConfiguration(&'static str),
}

impl fmt::Display for ProfilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfilerError::Io(e) => write!(f, "I/O error: {e}"),
            ProfilerError::Csv(e) => write!(f, "CSV error: {e}"),
            ProfilerError::Model(e) => write!(f, "model error: {e}"),
            ProfilerError::MissingProfiles => write!(f, "no profiles loaded"),
            ProfilerError::Serialization(e) => write!(f, "serialization error: {e}"),
            ProfilerError::InvalidConfiguration(message) => {
                write!(f, "invalid configuration: {message}")
            }
        }
    }
}

impl std::error::Error for ProfilerError {}

impl From<std::io::Error> for ProfilerError {
    fn from(value: std::io::Error) -> Self {
        ProfilerError::Io(value)
    }
}

impl From<csv::Error> for ProfilerError {
    fn from(value: csv::Error) -> Self {
        ProfilerError::Csv(value)
    }
}

impl From<ModelError> for ProfilerError {
    fn from(value: ModelError) -> Self {
        ProfilerError::Model(value)
    }
}

impl From<serde_json::Error> for ProfilerError {
    fn from(value: serde_json::Error) -> Self {
        ProfilerError::Serialization(value)
    }
}

#[derive(Debug, Serialize)]
struct PersistedModel<'a> {
    app: &'a str,
    #[serde(rename = "type")]
    model_type: &'a str,
    degree: Option<usize>,
    coefficients: Vec<f32>,
}

#[derive(Debug)]
pub struct Profiler {
    degree_of_polynomial: usize,
    profile_table: HashMap<String, Vec<Record>>,
    slowdown_table: HashMap<String, Vec<(f32, f32)>>,
    sensitivity_table: HashMap<String, Model>,
}

impl Profiler {
    pub fn new(degree_of_polynomial: usize) -> Self {
        Profiler {
            degree_of_polynomial,
            profile_table: HashMap::new(),
            slowdown_table: HashMap::new(),
            sensitivity_table: HashMap::new(),
        }
    }

    pub fn ingest_record(&mut self, record: Record) {
        let app = record.name().clone();
        self.profile_table.entry(app).or_default().push(record);
    }

    pub fn load_profile_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ProfilerError> {
        let mut reader = ReaderBuilder::new().trim(csv::Trim::All).from_path(path)?;
        for result in reader.deserialize() {
            let record: Record = result?;
            self.ingest_record(record);
        }
        if self.profile_table.is_empty() {
            return Err(ProfilerError::MissingProfiles);
        }
        Ok(())
    }

    pub fn build_slowdown_table(&mut self) -> Result<(), ProfilerError> {
        self.slowdown_table.clear();
        for (app, records) in &self.profile_table {
            let mut samples = Vec::with_capacity(records.len());
            for record in records {
                let bandwidth = record.bw() as f32 / 100.0;
                let completion_time = record.time() as f32;
                samples.push(CompletionSample::new(bandwidth, completion_time)?);
            }
            let slowdown = completion_samples_to_slowdown(&samples)?;
            debug!("Calculated slowdown curve for {}: {:?}", app, slowdown);
            self.slowdown_table.insert(app.clone(), slowdown);
        }
        Ok(())
    }

    pub fn build_sensitivity_table(&mut self) -> Result<(), ProfilerError> {
        if self.slowdown_table.is_empty() {
            self.build_slowdown_table()?;
        }

        self.sensitivity_table.clear();
        for (app, slowdown) in &self.slowdown_table {
            let mut curve_model =
                Model::SensitivityCurve(SensitivityCurve::new(self.degree_of_polynomial));
            match curve_model.fit(slowdown) {
                Ok(()) => {
                    self.sensitivity_table.insert(app.clone(), curve_model);
                }
                Err(ModelError::NotEnoughSamples { .. }) | Err(ModelError::SingularMatrix) => {
                    let mut score_model = Model::SensitivityScore(SensitivityScore { score: 0.0 });
                    score_model.fit(slowdown)?;
                    self.sensitivity_table.insert(app.clone(), score_model);
                }
                Err(err) => return Err(err.into()),
            }
        }
        Ok(())
    }

    pub fn write_sensitivity_table<P: AsRef<Path>>(&self, path: P) -> Result<(), ProfilerError> {
        let mut rows = Vec::with_capacity(self.sensitivity_table.len());
        for (app, model) in &self.sensitivity_table {
            match model {
                Model::SensitivityCurve(curve) => rows.push(PersistedModel {
                    app,
                    model_type: "curve",
                    degree: Some(curve.degree_of_polynomial),
                    coefficients: curve.coefficients.clone(),
                }),
                Model::SensitivityScore(score) => rows.push(PersistedModel {
                    app,
                    model_type: "score",
                    degree: None,
                    coefficients: vec![score.score],
                }),
            }
        }

        let mut file = File::create(path)?;
        serde_json::to_writer_pretty(&mut file, &rows)?;
        file.write_all(b"\n")?;
        Ok(())
    }

    pub fn sensitivity_table(&self) -> &HashMap<String, Model> {
        &self.sensitivity_table
    }
}

#[derive(Debug)]
pub struct ProfilingJob {
    pub applications: Vec<String>,
}

#[derive(Debug)]
pub struct ProfilingQueue {
    jobs: Mutex<Option<VecDeque<ProfilingJob>>>,
    cvar: Condvar,
}

impl ProfilingQueue {
    pub fn new() -> Self {
        ProfilingQueue {
            jobs: Mutex::new(Some(VecDeque::new())),
            cvar: Condvar::new(),
        }
    }

    pub fn profile(&self, unprofiled_applications: Vec<ProfilingJob>) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(queue) = jobs.as_mut() {
            queue.extend(unprofiled_applications);
            self.cvar.notify_all();
        }
    }

    pub fn wait_for_job(&self) -> Option<ProfilingJob> {
        let mut jobs = self.jobs.lock().unwrap();
        loop {
            match jobs.as_mut()?.pop_front() {
                Some(job) => return Some(job),
                None => jobs = self.cvar.wait(jobs).unwrap(),
            }
        }
    }

    pub fn end(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        *jobs = None;
        self.cvar.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    const CSV_FIXTURE: &str = "app,dataset_size,number_of_nodes,bw,time\napp1,1,1,100,10\napp1,1,1,75,15\napp1,1,1,50,25\n";

    #[test]
    fn builds_sensitivity_models_from_csv() -> Result<(), ProfilerError> {
        let dir = tempdir().unwrap();
        let csv_path = dir.path().join("profile.csv");
        fs::write(&csv_path, CSV_FIXTURE).unwrap();

        let mut profiler = Profiler::new(2);
        profiler.load_profile_csv(&csv_path)?;
        profiler.build_slowdown_table()?;
        profiler.build_sensitivity_table()?;

        let out_path = dir.path().join("sensitivity.json");
        profiler.write_sensitivity_table(&out_path)?;
        let json = fs::read_to_string(out_path).unwrap();
        assert!(json.contains("\"app1\""));
        Ok(())
    }

    #[test]
    fn falls_back_to_score_when_samples_insufficient() -> Result<(), ProfilerError> {
        let mut profiler = Profiler::new(4);
        profiler.ingest_record(Record::new("app_score".to_string(), 1, 1, 100, 10));
        profiler.build_slowdown_table()?;
        profiler.build_sensitivity_table()?;

        match profiler.sensitivity_table().get("app_score").unwrap() {
            Model::SensitivityScore(score) => assert!((score.score - 1.0).abs() < 1e-6),
            _ => panic!("expected score model fallback"),
        }
        Ok(())
    }
}

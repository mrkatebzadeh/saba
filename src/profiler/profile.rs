use crate::kmeans::*;
use crate::record::Record;
use saba::model::Model;
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Profiler {
    priorities: u8,
    profile_table: HashMap<String, Vec<Record>>,
    slowdown_table: HashMap<String, Vec<f32>>,
    sensitivity_table: HashMap<String, Model>,
}

/// Constructor for Profiler.
impl Profiler {
    pub fn new(priorities: u8) -> Self {
        Profiler {
            priorities,
            profile_table: HashMap::new(),
            slowdown_table: HashMap::new(),
            sensitivity_table: HashMap::new(),
        }
    }
}

impl Profiler {
    /// Returns the completion time of an application with unthrottled bandwidth.
    #[allow(dead_code)]
    fn get_baseline_completion_time(&self, app: &str) -> Option<u16> {
        let profile_table = self.profile_table.get(app)?;
        for record in profile_table {
            if record.bw() == 100 {
                return Some(record.time());
            }
        }
        None
    }

    /// Reads the profile table from a CSV file.
    #[allow(dead_code)]
    fn read_from_file(filename: &str) -> Result<HashMap<String, Vec<Record>>, String> {
        let mut profile_table: HashMap<String, Vec<Record>> = HashMap::new();
        let mut reader = csv::Reader::from_path(filename).map_err(|e| e.to_string())?;
        for result in reader.deserialize() {
            let record: Record = result.map_err(|e| e.to_string())?;
            if profile_table.contains_key(record.name()) {
                profile_table.get_mut(record.name()).unwrap().push(record);
            } else {
                profile_table.insert(record.name().clone(), vec![record]);
            }
        }
        Ok(profile_table)
    }

    /// Calculates the slowdown of an application for each bandwidth value.
    #[allow(dead_code)]
    fn fill_slowdown_table(&mut self) {
        for app in self.profile_table.keys() {
            let values: Vec<u16> = self.profile_table[app].iter().map(|r| r.time()).collect();
            let min_value = values.iter().min().unwrap();
            let mut slowdowns = Vec::new();

            for value in values.iter() {
                slowdowns.push(*value as f32 / *min_value as f32);
            }
            self.slowdown_table.insert(app.clone(), slowdowns);
        }
    }

    /// Clusters the applications based on their sensitivity.
    #[allow(dead_code)]
    fn cluster_applications(&self) {
        let mut table: Vec<Model> = Vec::new();
        for app in self.sensitivity_table.iter() {
            let model = app.1;
            table.push(model.clone());
        }

        // convert table to a matrix
        let mut matrix: Vec<Vec<f32>> = Vec::new();

        // let kmeans = KMeans::new(table, table.len(), 3);
        // todo!(); // Parameterize the number of dimensions.

        //
    }
}

pub struct ProfilingJob {
    pub applications: Vec<String>,
}

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
}

impl ProfilingQueue {
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

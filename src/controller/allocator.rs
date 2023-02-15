//! This module contains the allocator implementation.
//! List of implemented allocators:
//! - SabaAllocator
//! - MaxMinAllocator
//!

use crate::model::Model;
use crate::profile::ProfileRecord;
use log::debug;
use std::{collections::HashMap, fmt::Debug};

/// Allocator is a trait that defines the interface for the allocator.
pub trait Allocator: Debug {
    fn allocate(&mut self);
}

/// SabaAllocator is an allocator that uses the Saba scheme.
/// Saba is a bandwidth allocation scheme that uses a sensitivity model to
/// predict the slowdown of an application when the bandwidth is reduced.
/// The algorithm is described in the paper "Saba: Rethinking Datacenter Network
/// Allocation from Application’s Perspective" by M.R.S. Katebzadeh et al.
/// The algorithm is implemented in the `allocate` method.
/// The allocator uses the following tables:
/// - profile_table: a table that contains the profile of each application.
/// - slowdown_table: a table that contains the slowdown of each application
///  when the bandwidth is reduced.
///  The slowdown is calculated for each bandwidth value using the following formula:
///  slowdown = completion_time / baseline_completion_time
///  where completion_time is the completion time of the application when the
///  bandwidth is reduced, and baseline_completion_time is the completion time
///  of the application with unthrottled bandwidth.
///
#[derive(Debug)]
pub struct SabaAllocator<Sensitivity: Model> {
    profile_table: HashMap<String, Vec<ProfileRecord>>,
    slowdown_table: HashMap<String, Vec<f32>>,
    sensitivity_table: HashMap<String, Box<dyn Model<Other = Sensitivity>>>,
}

/// Trait implementation for SabaAllocator.
impl<Sensitivity: Model> Allocator for SabaAllocator<Sensitivity> {
    fn allocate(&mut self) {
        debug!("Allocating with Saba..");
        unimplemented!()
    }
}

/// Constructor for SabaAllocator.
impl<Sensitivity: Model> SabaAllocator<Sensitivity> {
    pub fn new() -> Self {
        SabaAllocator {
            profile_table: HashMap::new(),
            slowdown_table: HashMap::new(),
            sensitivity_table: HashMap::new(),
        }
    }
}

impl<Sensitivity: Model> SabaAllocator<Sensitivity> {
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
    fn read_from_file(filename: &str) -> Result<HashMap<String, Vec<ProfileRecord>>, String> {
        let mut profile_table: HashMap<String, Vec<ProfileRecord>> = HashMap::new();
        let mut reader = csv::Reader::from_path(filename).map_err(|e| e.to_string())?;
        for result in reader.deserialize() {
            let record: ProfileRecord = result.map_err(|e| e.to_string())?;
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
        let mut table: Vec<&Box<dyn Model<Other = Sensitivity>>> = Vec::new();
        for app in self.sensitivity_table.iter() {
            let model = app.1;
            table.push(model);
        }
    }
}

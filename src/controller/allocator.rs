use crate::profile::ProfileRecord;
use log::debug;
use std::{collections::HashMap, fmt::Debug};

pub trait Allocator: Debug {
    fn allocate(&mut self);
}

#[derive(Debug)]
pub struct SabaAllocator {
    profile_table: HashMap<String, Vec<ProfileRecord>>,
    slowdown_table: HashMap<String, Vec<f32>>,
    sensitivity_table: HashMap<String, f32>,
}

impl Allocator for SabaAllocator {
    fn allocate(&mut self) {
        debug!("Allocating with Saba..");
        unimplemented!()
    }
}

impl SabaAllocator {
    pub fn new() -> Self {
        SabaAllocator {
            profile_table: HashMap::new(),
            slowdown_table: HashMap::new(),
            sensitivity_table: HashMap::new(),
        }
    }
}

impl SabaAllocator {
    #[allow(dead_code)]
    fn get_time_with_unthrottled_bw(&self, app: &str) -> Option<u16> {
        let profile_table = self.profile_table.get(app)?;
        for record in profile_table {
            if record.bw() == 100 {
                return Some(record.time());
            }
        }
        None
    }

    #[allow(dead_code)]
    fn read_profile_table_from_file(
        filename: &str,
    ) -> Result<HashMap<String, Vec<ProfileRecord>>, String> {
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

    #[allow(dead_code)]
    fn generate_slowdown_table(&mut self) {
        for app in self.profile_table.keys() {
            self.slowdown_table
                .insert(app.clone(), self.get_slowdown(app).unwrap());
        }
    }

    #[allow(dead_code)]
    fn cluster_applications(&self) {
        let mut table: Vec<Vec<f32>> = Vec::new();
        for app in self.slowdown_table.iter().sorted_by(|a, b| a.0.cmp(b.0)) {
            let row: Vec<f32> = app.1.clone();
            table.push(row);
        }
    }
}

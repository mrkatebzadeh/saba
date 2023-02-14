use crate::model::Model;
use crate::profile::ProfileRecord;
use log::debug;
use std::{collections::HashMap, fmt::Debug};

pub trait Allocator: Debug {
    fn allocate(&mut self);
}

#[derive(Debug)]
pub struct SabaAllocator<Sensitivity: Model> {
    profile_table: HashMap<String, Vec<ProfileRecord>>,
    slowdown_table: HashMap<String, Vec<f32>>,
    sensitivity_table: HashMap<String, Box<dyn Model<Other = Sensitivity>>>,
}

impl<Sensitivity: Model> Allocator for SabaAllocator<Sensitivity> {
    fn allocate(&mut self) {
        debug!("Allocating with Saba..");
        unimplemented!()
    }
}

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

    #[allow(dead_code)]
    fn cluster_applications(&self) {
        let mut table: Vec<&Box<dyn Model<Other = Sensitivity>>> = Vec::new();
        for app in self.sensitivity_table.iter() {
            let model = app.1;
            table.push(model);
        }
    }
}

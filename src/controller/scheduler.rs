use crate::connection::Connection;
use crate::profile::BandwidthValuePercent;
use crate::profile::ProfileRecord;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AllocationAlgorithm {
    InfiniBand,
    IdealMaxMin,
    SabaHierarchical,
    SabaMean,
}

impl AllocationAlgorithm {
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Result<AllocationAlgorithm, String> {
        match s {
            "infiniband" => Ok(AllocationAlgorithm::InfiniBand),
            "idealmaxmin" => Ok(AllocationAlgorithm::IdealMaxMin),
            "sabahierarchical" => Ok(AllocationAlgorithm::SabaHierarchical),
            "sabamean" => Ok(AllocationAlgorithm::SabaMean),
            _ => Err(format!("Unknown allocation algorithm: {s}")),
        }
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> &str {
        match self {
            AllocationAlgorithm::InfiniBand => "infiniband",
            AllocationAlgorithm::IdealMaxMin => "idealmaxmin",
            AllocationAlgorithm::SabaHierarchical => "sabahierarchical",
            AllocationAlgorithm::SabaMean => "sabamean",
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct AllocationRecord {
    app: String,
    src: String,
    dst: String,
    bw: f32,
    priority: u8,
}

impl AllocationRecord {
    #[allow(dead_code)]
    fn new(app: String, src: String, dst: String, bw: f32, priority: u8) -> Self {
        AllocationRecord {
            app,
            src,
            dst,
            bw,
            priority,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Scheduler {
    allocation_algorithm: AllocationAlgorithm,
    priority_levels: u8,

    //app,dataset_size,number_of_nodes,bw,time
    profile_table: HashMap<String, Vec<ProfileRecord>>,
    //app,src,dst,bw,priority
    allocation_table: Vec<AllocationRecord>,

    slowdown_table: HashMap<String, Vec<f32>>,
    sensitivity_table: HashMap<String, f32>,
    priority_to_app_table: HashMap<u8, Vec<String>>,
    connection_to_app_table: HashMap<Connection, String>,
    app_to_priority_table: HashMap<String, u8>,
}

impl Scheduler {
    #[allow(dead_code)]
    pub fn new(
        allocation_algorithm: AllocationAlgorithm,
        priority_levels: u8,
        profile_table: HashMap<String, Vec<ProfileRecord>>,
    ) -> Self {
        Scheduler {
            allocation_algorithm,
            priority_levels,
            profile_table,
            allocation_table: Vec::new(),
            slowdown_table: HashMap::new(),
            sensitivity_table: HashMap::new(),
            priority_to_app_table: HashMap::new(),
            connection_to_app_table: HashMap::new(),
            app_to_priority_table: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn allocate(&mut self) {
        match self.allocation_algorithm {
            AllocationAlgorithm::InfiniBand => self.allocate_infiniband(),
            AllocationAlgorithm::IdealMaxMin => self.allocate_idealmaxmin(),
            AllocationAlgorithm::SabaHierarchical => self.allocate_sabahierarchical(),
            AllocationAlgorithm::SabaMean => self.allocate_sabamean(),
        }
    }

    #[allow(dead_code)]
    pub fn slowdown(&self, app: &str) -> Option<&Vec<f32>> {
        self.slowdown_table.get(app)
    }

    #[allow(dead_code)]
    pub fn sensitivity(&self, app: &str) -> Option<&f32> {
        self.sensitivity_table.get(app)
    }

    #[allow(dead_code)]
    pub fn priority_to_app(&self, priority: u8) -> Option<&Vec<String>> {
        self.priority_to_app_table.get(&priority)
    }

    #[allow(dead_code)]
    pub fn connection_to_app(&self, connection: &Connection) -> Option<&String> {
        self.connection_to_app_table.get(connection)
    }

    #[allow(dead_code)]
    pub fn app_to_priority(&self, app: &str) -> Option<&u8> {
        self.app_to_priority_table.get(app)
    }

    #[allow(dead_code)]
    pub fn allocation_table(&self) -> &Vec<AllocationRecord> {
        &self.allocation_table
    }

    fn allocate_infiniband(&mut self) {
        unimplemented!(); // TODO
    }

    fn allocate_idealmaxmin(&mut self) {
        unimplemented!(); // TODO
    }

    fn allocate_sabahierarchical(&mut self) {
        unimplemented!(); // TODO
    }

    fn allocate_sabamean(&mut self) {
        unimplemented!(); // TODO
    }

    #[allow(dead_code)]
    pub fn read_profile_table_from_file(
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
    fn get_time_with_unthrottled_bw(&self, app: &str) -> Option<u16> {
        let profile_table = self.profile_table.get(app)?;
        for record in profile_table {
            if *record.bw() == BandwidthValuePercent::Hundred {
                return Some(record.time());
            }
        }
        None
    }

    #[allow(dead_code)]
    fn get_slowdown(&self, app: &str) -> Option<Vec<f32>> {
        let profile_table = self.profile_table.get(app)?;
        let mut slowdown = Vec::new();
        for record in profile_table {
            let time_with_unthrottled_bw = self.get_time_with_unthrottled_bw(app)?;
            let slowdown_value = time_with_unthrottled_bw as f32 / record.time() as f32;
            slowdown.push((100.0 * slowdown_value).round() / 100.0);
        }
        Some(slowdown)
    }

    #[allow(dead_code)]
    pub fn generate_slowdown_table(&mut self) {
        for app in self.profile_table.keys() {
            self.slowdown_table
                .insert(app.clone(), self.get_slowdown(app).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::profile::BandwidthValuePercent;

    use super::*;

    #[test]
    fn test_allocation_algorithm_from_str() {
        assert_eq!(
            AllocationAlgorithm::from_str("infiniband").unwrap(),
            AllocationAlgorithm::InfiniBand
        );
        assert_eq!(
            AllocationAlgorithm::from_str("idealmaxmin").unwrap(),
            AllocationAlgorithm::IdealMaxMin
        );
        assert_eq!(
            AllocationAlgorithm::from_str("sabahierarchical").unwrap(),
            AllocationAlgorithm::SabaHierarchical
        );
        assert_eq!(
            AllocationAlgorithm::from_str("sabamean").unwrap(),
            AllocationAlgorithm::SabaMean
        );
        assert!(AllocationAlgorithm::from_str("unknown").is_err());
    }

    #[test]
    fn test_allocation_algorithm_to_str() {
        assert_eq!(AllocationAlgorithm::InfiniBand.to_str(), "infiniband");
        assert_eq!(AllocationAlgorithm::IdealMaxMin.to_str(), "idealmaxmin");
        assert_eq!(
            AllocationAlgorithm::SabaHierarchical.to_str(),
            "sabahierarchical"
        );
        assert_eq!(AllocationAlgorithm::SabaMean.to_str(), "sabamean");
    }

    #[test]
    fn test_scheduler_new() {
        let allocation_algorithm = AllocationAlgorithm::InfiniBand;
        let priority_levels = 3;
        let profile_table: HashMap<String, Vec<ProfileRecord>> = [
            (
                "app1".to_string(),
                vec![ProfileRecord::new(
                    "app1".to_string(),
                    1,
                    2,
                    crate::profile::BandwidthValuePercent::Ten,
                    1,
                )],
            ),
            (
                "app2".to_string(),
                vec![ProfileRecord::new(
                    "app1".to_string(),
                    1,
                    2,
                    crate::profile::BandwidthValuePercent::Ten,
                    2,
                )],
            ),
        ]
        .iter()
        .cloned()
        .collect();
        let scheduler =
            Scheduler::new(allocation_algorithm.clone(), priority_levels, profile_table);
        assert_eq!(scheduler.allocation_algorithm, allocation_algorithm);
        assert_eq!(scheduler.priority_levels, priority_levels);
        assert_eq!(scheduler.profile_table.len(), 2);
        assert_eq!(scheduler.allocation_table.len(), 0);
        assert_eq!(scheduler.slowdown_table.len(), 0);
        assert_eq!(scheduler.sensitivity_table.len(), 0);
        assert_eq!(scheduler.priority_to_app_table.len(), 0);
        assert_eq!(scheduler.connection_to_app_table.len(), 0);
        assert_eq!(scheduler.app_to_priority_table.len(), 0);
    }

    #[test]
    fn test_read_profile_table_from_file() {
        let filename = "tests/profile.csv";
        let profile_table = Scheduler::read_profile_table_from_file(filename).unwrap();
        assert_eq!(profile_table.len(), 1);
        assert_eq!(profile_table["app1"].len(), 3);
        assert_eq!(profile_table["app1"][0].name(), "app1");
        assert_eq!(*profile_table["app1"][0].bw(), BandwidthValuePercent::Ten);
        assert_eq!(profile_table["app1"][0].time(), 3);
        assert_eq!(profile_table["app1"][0].dataset_size(), 1);
        assert_eq!(profile_table["app1"][0].number_of_nodes(), 1);

        assert_eq!(profile_table["app1"][1].name(), "app1");
        assert_eq!(
            *profile_table["app1"][1].bw(),
            BandwidthValuePercent::TwentyFive
        );
        assert_eq!(profile_table["app1"][1].time(), 2);
        assert_eq!(profile_table["app1"][1].dataset_size(), 1);
        assert_eq!(profile_table["app1"][1].number_of_nodes(), 1);

        assert_eq!(profile_table["app1"][2].name(), "app1");
        assert_eq!(
            *profile_table["app1"][2].bw(),
            BandwidthValuePercent::Hundred
        );
        assert_eq!(profile_table["app1"][2].time(), 1);
        assert_eq!(profile_table["app1"][2].dataset_size(), 1);
        assert_eq!(profile_table["app1"][2].number_of_nodes(), 1);
    }

    #[test]
    fn test_get_time_with_unthrottled_bw() {
        let filename = "tests/profile.csv";
        let profile_table = Scheduler::read_profile_table_from_file(filename).unwrap();
        let scheduler = Scheduler::new(AllocationAlgorithm::InfiniBand, 3, profile_table);
        assert_eq!(scheduler.get_time_with_unthrottled_bw("app1"), Some(1));
        assert_eq!(scheduler.get_time_with_unthrottled_bw("app2"), None);
    }

    #[test]
    fn test_get_slowdown() {
        let filename = "tests/profile.csv";
        let profile_table = Scheduler::read_profile_table_from_file(filename).unwrap();
        let scheduler = Scheduler::new(AllocationAlgorithm::InfiniBand, 3, profile_table);
        assert_eq!(scheduler.get_slowdown("app1"), Some(vec![0.33, 0.5, 1.0]));
        assert_eq!(scheduler.get_slowdown("app2"), None);
    }

    #[test]
    fn test_generate_slowdown_table() {
        let filename = "tests/profile.csv";
        let profile_table = Scheduler::read_profile_table_from_file(filename).unwrap();
        let mut scheduler = Scheduler::new(AllocationAlgorithm::InfiniBand, 3, profile_table);
        scheduler.generate_slowdown_table();
        assert_eq!(scheduler.slowdown_table.len(), 1);
        assert_eq!(scheduler.slowdown_table["app1"].len(), 3);
        assert_eq!(scheduler.slowdown_table["app1"][0], 0.33);
        assert_eq!(scheduler.slowdown_table["app1"][1], 0.5);
        assert_eq!(scheduler.slowdown_table["app1"][2], 1.0);
    }
}

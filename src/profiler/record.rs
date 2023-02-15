use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    app: String,
    dataset_size: u16,
    number_of_nodes: u16,
    bw: u16,
    time: u16,
}

impl Record {
    #[allow(dead_code)]
    pub fn new(app: String, dataset_size: u16, number_of_nodes: u16, bw: u16, time: u16) -> Self {
        Record {
            app,
            dataset_size,
            number_of_nodes,
            bw,
            time,
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &String {
        &self.app
    }

    #[allow(dead_code)]
    pub fn dataset_size(&self) -> u16 {
        self.dataset_size
    }

    #[allow(dead_code)]
    pub fn number_of_nodes(&self) -> u16 {
        self.number_of_nodes
    }

    #[allow(dead_code)]
    pub fn bw(&self) -> u16 {
        self.bw
    }

    #[allow(dead_code)]
    pub fn time(&self) -> u16 {
        self.time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_record() {
        let record = Record::new("app".to_string(), 100, 10, 50, 1000);
        assert_eq!(record.name(), &"app".to_string());
        assert_eq!(record.dataset_size(), 100);
        assert_eq!(record.number_of_nodes(), 10);
        assert_eq!(record.bw(), 50);
        assert_eq!(record.time(), 1000);
    }
}

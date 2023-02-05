use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub enum BandwidthValuePercent {
    Five,
    Ten,
    TwentyFive,
    Fifty,
    SeventyFive,
    Ninety,
    Hundred,
}

impl BandwidthValuePercent {
    pub fn value(&self) -> u8 {
        match self {
            BandwidthValuePercent::Five => 5,
            BandwidthValuePercent::Ten => 10,
            BandwidthValuePercent::TwentyFive => 25,
            BandwidthValuePercent::Fifty => 50,
            BandwidthValuePercent::SeventyFive => 75,
            BandwidthValuePercent::Ninety => 90,
            BandwidthValuePercent::Hundred => 100,
        }
    }
}

impl PartialEq for BandwidthValuePercent {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}
impl Eq for BandwidthValuePercent {}

#[derive(Debug, Deserialize, Clone)]
pub struct ProfileRecord {
    app: String,
    dataset_size: u16,
    number_of_nodes: u16,
    bw: BandwidthValuePercent,
    time: u16,
}

impl ProfileRecord {
    #[allow(dead_code)]
    pub fn new(
        app: String,
        dataset_size: u16,
        number_of_nodes: u16,
        bw: BandwidthValuePercent,
        time: u16,
    ) -> Self {
        ProfileRecord {
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
    pub fn bw(&self) -> &BandwidthValuePercent {
        &self.bw
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
    fn test_bandwidth_value_percent() {
        assert_eq!(BandwidthValuePercent::Five.value(), 5);
        assert_eq!(BandwidthValuePercent::Ten.value(), 10);
        assert_eq!(BandwidthValuePercent::TwentyFive.value(), 25);
        assert_eq!(BandwidthValuePercent::Fifty.value(), 50);
        assert_eq!(BandwidthValuePercent::SeventyFive.value(), 75);
        assert_eq!(BandwidthValuePercent::Ninety.value(), 90);
        assert_eq!(BandwidthValuePercent::Hundred.value(), 100);
    }

    #[test]
    fn test_profile_record() {
        let profile_record = ProfileRecord::new(
            "app".to_string(),
            100,
            10,
            BandwidthValuePercent::Fifty,
            1000,
        );
        assert_eq!(profile_record.name(), &"app".to_string());
        assert_eq!(profile_record.dataset_size(), 100);
        assert_eq!(profile_record.number_of_nodes(), 10);
        assert_eq!(profile_record.bw(), &BandwidthValuePercent::Fifty);
        assert_eq!(profile_record.time(), 1000);
    }
}

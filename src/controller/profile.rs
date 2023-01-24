#[derive(Debug)]
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

#[derive(Debug)]
pub struct Profile {
    name: String,
    dataset_size: u16,
    number_of_nodes: u16,
    weights: Vec<(BandwidthValuePercent, f32)>,
}

impl Profile {
    #[allow(dead_code)]
    pub fn new(
        name: String,
        dataset_size: u16,
        number_of_nodes: u16,
        weights: Vec<(BandwidthValuePercent, f32)>,
    ) -> Self {
        Profile {
            name,
            dataset_size,
            number_of_nodes,
            weights,
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &String {
        &self.name
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
    pub fn weights(&self) -> &Vec<(BandwidthValuePercent, f32)> {
        &self.weights
    }

    #[allow(dead_code)]
    pub fn get_weights_as_vector(&self) -> Vec<f32> {
        let mut weights = Vec::new();
        for (_, weight) in self.weights.iter() {
            weights.push(*weight);
        }
        weights
    }

    #[allow(dead_code)]
    pub fn get_bandwidth_values(&self) -> Vec<u8> {
        let mut bandwidth_values = Vec::new();
        for (bandwidth_value, _) in self.weights.iter() {
            bandwidth_values.push(bandwidth_value.value());
        }
        bandwidth_values
    }

    #[allow(dead_code)]
    pub fn get_coefficients(&self) -> Vec<(u8, f32)> {
        let mut coefficients = Vec::new();
        for (bandwidth, weight) in self.weights.iter() {
            coefficients.push((bandwidth.value(), *weight));
        }
        coefficients
    }
}

#[allow(dead_code)]
fn compare_vecs(
    vec1: &Vec<(BandwidthValuePercent, f32)>,
    vec2: &Vec<(BandwidthValuePercent, f32)>,
) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }
    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile() {
        let profile = Profile::new(
            "test".to_string(),
            100,
            10,
            vec![
                (BandwidthValuePercent::Five, 0.1),
                (BandwidthValuePercent::Ten, 0.2),
                (BandwidthValuePercent::TwentyFive, 0.3),
                (BandwidthValuePercent::Fifty, 0.4),
                (BandwidthValuePercent::SeventyFive, 0.5),
                (BandwidthValuePercent::Ninety, 0.6),
                (BandwidthValuePercent::Hundred, 1.0),
            ],
        );
        assert_eq!(profile.name(), "test");
        assert_eq!(profile.dataset_size(), 100);
        assert_eq!(profile.number_of_nodes(), 10);
        assert!(compare_vecs(
            profile.weights(),
            &vec![
                (BandwidthValuePercent::Five, 0.1),
                (BandwidthValuePercent::Ten, 0.2),
                (BandwidthValuePercent::TwentyFive, 0.3),
                (BandwidthValuePercent::Fifty, 0.4),
                (BandwidthValuePercent::SeventyFive, 0.5),
                (BandwidthValuePercent::Ninety, 0.6),
                (BandwidthValuePercent::Hundred, 1.0),
            ]
        ));
        assert_eq!(
            profile.get_weights_as_vector(),
            vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 1.0]
        );
        assert_eq!(
            profile.get_bandwidth_values(),
            vec![
                BandwidthValuePercent::Five.value(),
                BandwidthValuePercent::Ten.value(),
                BandwidthValuePercent::TwentyFive.value(),
                BandwidthValuePercent::Fifty.value(),
                BandwidthValuePercent::SeventyFive.value(),
                BandwidthValuePercent::Ninety.value(),
                BandwidthValuePercent::Hundred.value(),
            ]
        );
        assert_eq!(
            profile.get_coefficients(),
            vec![
                (5, 0.1),
                (10, 0.2),
                (25, 0.3),
                (50, 0.4),
                (75, 0.5),
                (90, 0.6),
                (100, 1.0),
            ]
        );
    }
}

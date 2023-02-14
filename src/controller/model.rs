use crate::profile::ProfileRecord;

trait Model {
    type Other;
    fn slowdown(&self, bw: f32) -> f32;
    fn distance(&self, other: &Self::Other) -> f32;
    fn fit(&self, records: &Vec<ProfileRecord>);
}

struct SensitivityCurve {
    pub coefficients: Vec<f32>,
}

impl Model for SensitivityCurve {
    type Other = SensitivityCurve;
    fn slowdown(&self, bw: f32) -> f32 {
        let mut slowdown = 0.0;
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            slowdown += coefficient * bw.powi(i as i32);
        }
        slowdown
    }

    fn distance(&self, other: &SensitivityCurve) -> f32 {
        let mut distance = 0.0;
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            distance += (coefficient - other.coefficients[i]).powi(2);
        }
        distance.sqrt()
    }

    fn fit(&self, records: &Vec<ProfileRecord>) {
        let mut values: Vec<u16> = records.iter().map(|r| r.time()).collect();
        let min_value = values.iter().min().unwrap();

        for value in values.iter_mut() {
            *value = *value / min_value;
        }
        unimplemented!()
    }
}

struct SensitivityScore {
    pub score: f32,
}

impl Model for SensitivityScore {
    type Other = SensitivityScore;

    fn slowdown(&self, bw: f32) -> f32 {
        self.score
    }

    fn distance(&self, other: &Self::Other) -> f32 {
        (self.score - other.score).abs()
    }

    fn fit(&self, records: &Vec<ProfileRecord>) {
        let mut values: Vec<u16> = records.iter().map(|r| r.time()).collect();
        let min_value = values.iter().min().unwrap();

        for value in values.iter_mut() {
            *value = *value / min_value;
        }

        let score = values.iter().sum::<u16>() as f32 / values.len() as f32;
        self.score = score;
    }
}

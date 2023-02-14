use crate::profile::ProfileRecord;
use std::fmt::Debug;

pub trait Model: Debug {
    type Other;
    fn slowdown(&self, bw: f32) -> f32;
    fn distance(&self, other: &Self::Other) -> f32;
    fn fit(&mut self, records: &Vec<f32>);
}

#[derive(Debug)]
pub struct SensitivityCurve {
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

    fn fit(&mut self, records: &Vec<f32>) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct SensitivityScore {
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

    fn fit(&mut self, records: &Vec<f32>) {
        let score = 0.0;
        self.score = score;
    }
}

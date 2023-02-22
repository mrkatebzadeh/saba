use nalgebra::{DMatrix, DVector};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub enum Model {
    SensitivityCurve(SensitivityCurve),
    SensitivityScore(SensitivityScore),
}

impl Model {
    pub fn slowdown(&self, bw: f32) -> f32 {
        match self {
            Model::SensitivityCurve(curve) => curve.slowdown(bw),
            Model::SensitivityScore(score) => score.slowdown(bw),
        }
    }

    pub fn parameters(&self) -> Vec<f32> {
        match self {
            Model::SensitivityCurve(curve) => curve.parameters(),
            Model::SensitivityScore(score) => score.parameters(),
        }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        match self {
            Model::SensitivityCurve(curve) => curve.distance(other),
            Model::SensitivityScore(score) => score.distance(other),
        }
    }

    pub fn fit(&mut self, records: &HashMap<u32, f32>) {
        match self {
            Model::SensitivityCurve(curve) => curve.fit(records),
            Model::SensitivityScore(score) => score.fit(records),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        match self {
            Model::SensitivityCurve(curve) => curve.add(other),
            Model::SensitivityScore(score) => score.add(other),
        }
    }

    pub fn divide(&self, scalar: f32) -> Self {
        match self {
            Model::SensitivityCurve(curve) => curve.divide(scalar),
            Model::SensitivityScore(score) => score.divide(scalar),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SensitivityCurve {
    pub coefficients: Vec<f32>,
    pub degree_of_polynomial: usize,
}

impl SensitivityCurve {
    fn slowdown(&self, bw: f32) -> f32 {
        let mut slowdown = 0.0;
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            slowdown += coefficient * bw.powi(i as i32);
        }
        slowdown
    }

    fn distance(&self, other: &Model) -> f32 {
        let mut distance = 0.0;
        let other = match other {
            Model::SensitivityCurve(curve) => curve,
            _ => panic!("Invalid model type"),
        };
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            distance += (coefficient - other.coefficients[i]).powi(2);
        }
        distance.sqrt()
    }

    fn fit(&mut self, records: &HashMap<u32, f32>) {
        let n = records.len();
        let mut x = DMatrix::<f32>::zeros(n, self.degree_of_polynomial + 1);

        for (i, &value) in records.values().enumerate() {
            for j in 0..=self.degree_of_polynomial {
                x[(i, j)] = value.powf(j as f32);
            }
        }

        let y: Vec<f32> = records.values().cloned().collect();
        let y = DVector::from(y);

        // Solve for coefficients: X * coefficients = y
        let svd = x.clone().svd(true, true);
        let svd_result = svd.solve(&y, self.degree_of_polynomial as f32);
        match svd_result {
            Ok(coefficients) => {
                self.coefficients = coefficients.data.into();
            }
            Err(_) => {
                println!("Failed to fit the model. Singular matrix or other issue.");
            }
        }
        unimplemented!()
    }

    fn parameters(&self) -> Vec<f32> {
        self.coefficients.clone()
    }

    fn add(&self, other: &Model) -> Model {
        let other = match other {
            Model::SensitivityCurve(curve) => curve,
            _ => panic!("Invalid model type"),
        };
        let mut coefficients = vec![];
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            coefficients.push(coefficient + other.coefficients[i]);
        }
        Model::SensitivityCurve(SensitivityCurve {
            coefficients,
            degree_of_polynomial: self.degree_of_polynomial,
        })
    }

    fn divide(&self, scalar: f32) -> Model {
        let mut coefficients = vec![];
        for coefficient in self.coefficients.iter() {
            coefficients.push(coefficient / scalar);
        }
        Model::SensitivityCurve(SensitivityCurve {
            coefficients,
            degree_of_polynomial: self.degree_of_polynomial,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SensitivityScore {
    pub score: f32,
}

impl SensitivityScore {
    fn slowdown(&self, _: f32) -> f32 {
        self.score
    }

    fn distance(&self, other: &Model) -> f32 {
        let other = match other {
            Model::SensitivityScore(score) => score,
            _ => panic!("Invalid model type"),
        };
        (self.score - other.score).abs()
    }

    fn fit(&mut self, records: &HashMap<u32, f32>) {
        self.score = records.values().sum::<f32>() / records.len() as f32;
    }

    fn parameters(&self) -> Vec<f32> {
        vec![self.score]
    }

    fn add(&self, other: &Model) -> Model {
        let other = match other {
            Model::SensitivityScore(score) => score,
            _ => panic!("Invalid model type"),
        };

        Model::SensitivityScore(SensitivityScore {
            score: self.score + other.score,
        })
    }

    fn divide(&self, scalar: f32) -> Model {
        Model::SensitivityScore(SensitivityScore {
            score: self.score / scalar,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit() {
        let mut model = SensitivityCurve {
            degree_of_polynomial: 2,
            coefficients: Vec::new(),
        };
        let records = HashMap::from([(10, 1.0), (20, 2.0), (30, 3.0), (40, 4.0), (100, 5.0)]);

        model.fit(&records);

        let coefficients = model.coefficients;
        assert_eq!(coefficients.len(), 3);
        assert_eq!(coefficients, [-0.0009, 0.1437, -0.4244]);
    }
}

use std::fmt::Debug;

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

    pub fn fit(&mut self, records: &Vec<f32>) {
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

    fn fit(&mut self, records: &Vec<f32>) {
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
        Model::SensitivityCurve(SensitivityCurve { coefficients })
    }

    fn divide(&self, scalar: f32) -> Model {
        let mut coefficients = vec![];
        for coefficient in self.coefficients.iter() {
            coefficients.push(coefficient / scalar);
        }
        Model::SensitivityCurve(SensitivityCurve { coefficients })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SensitivityScore {
    pub score: f32,
}

impl SensitivityScore {
    fn slowdown(&self, bw: f32) -> f32 {
        self.score
    }

    fn distance(&self, other: &Model) -> f32 {
        let other = match other {
            Model::SensitivityScore(score) => score,
            _ => panic!("Invalid model type"),
        };
        (self.score - other.score).abs()
    }

    fn fit(&mut self, records: &Vec<f32>) {
        let score = 0.0;
        self.score = score;
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

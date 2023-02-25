/* model.rs

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use nalgebra::{DMatrix, DVector};
use std::{cmp::Ordering, error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelError {
    NotEnoughSamples { needed: usize, provided: usize },
    MissingBaselineSample,
    InvalidBandwidth,
    InvalidCompletionTime,
    SingularMatrix,
    EmptySamples,
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::NotEnoughSamples { needed, provided } => {
                write!(
                    f,
                    "not enough samples: needed {needed}, provided {provided}"
                )
            }
            ModelError::MissingBaselineSample => write!(f, "missing baseline sample"),
            ModelError::InvalidBandwidth => write!(f, "invalid bandwidth measurement"),
            ModelError::InvalidCompletionTime => write!(f, "invalid completion time measurement"),
            ModelError::SingularMatrix => write!(f, "failed to solve regression (singular matrix)"),
            ModelError::EmptySamples => write!(f, "no samples provided"),
        }
    }
}

impl Error for ModelError {}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn derivative(&self, bw: f32) -> f32 {
        match self {
            Model::SensitivityCurve(curve) => curve.derivative(bw),
            Model::SensitivityScore(score) => score.derivative(bw),
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

    pub fn fit(&mut self, samples: &[(f32, f32)]) -> Result<(), ModelError> {
        match self {
            Model::SensitivityCurve(curve) => curve.fit(samples),
            Model::SensitivityScore(score) => score.fit(samples),
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

#[derive(Debug, Clone, PartialEq)]
pub struct SensitivityCurve {
    pub coefficients: Vec<f32>,
    pub degree_of_polynomial: usize,
}

impl SensitivityCurve {
    pub fn new(degree_of_polynomial: usize) -> Self {
        Self {
            coefficients: vec![0.0; degree_of_polynomial + 1],
            degree_of_polynomial,
        }
    }

    fn slowdown(&self, bw: f32) -> f32 {
        let mut slowdown = 0.0;
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            slowdown += coefficient * bw.powi(i as i32);
        }
        slowdown
    }

    fn derivative(&self, bw: f32) -> f32 {
        let mut derivative = 0.0;
        for (i, coefficient) in self.coefficients.iter().enumerate().skip(1) {
            derivative += *coefficient * i as f32 * bw.powi((i - 1) as i32);
        }
        derivative
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

    fn fit(&mut self, samples: &[(f32, f32)]) -> Result<(), ModelError> {
        let needed = self.degree_of_polynomial + 1;
        if samples.len() < needed {
            return Err(ModelError::NotEnoughSamples {
                needed,
                provided: samples.len(),
            });
        }

        let mut vandermonde = DMatrix::<f32>::zeros(samples.len(), needed);
        for (row, (bw_ratio, _)) in samples.iter().enumerate() {
            vandermonde[(row, 0)] = 1.0;
            for col in 1..needed {
                vandermonde[(row, col)] = bw_ratio.powi(col as i32);
            }
        }

        let y = DVector::from_vec(samples.iter().map(|(_, slowdown)| *slowdown).collect());
        let svd = vandermonde.svd(true, true);
        match svd.solve(&y, 1e-6) {
            Ok(solution) => {
                self.coefficients = solution.iter().copied().collect();
                Ok(())
            }
            Err(_) => Err(ModelError::SingularMatrix),
        }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensitivityScore {
    pub score: f32,
}

impl SensitivityScore {
    fn slowdown(&self, _: f32) -> f32 {
        self.score
    }

    fn derivative(&self, _bw: f32) -> f32 {
        0.0
    }

    fn distance(&self, other: &Model) -> f32 {
        let other = match other {
            Model::SensitivityScore(score) => score,
            _ => panic!("Invalid model type"),
        };
        (self.score - other.score).abs()
    }

    fn fit(&mut self, samples: &[(f32, f32)]) -> Result<(), ModelError> {
        if samples.is_empty() {
            return Err(ModelError::EmptySamples);
        }
        let slowdown_sum: f32 = samples.iter().map(|(_, slowdown)| *slowdown).sum();
        self.score = slowdown_sum / samples.len() as f32;
        Ok(())
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompletionSample {
    bandwidth: f32,
    completion_time_ms: f32,
}

impl CompletionSample {
    pub fn new(bandwidth: f32, completion_time_ms: f32) -> Result<Self, ModelError> {
        let sample = Self {
            bandwidth,
            completion_time_ms,
        };
        sample.validate()?;
        Ok(sample)
    }

    pub fn bandwidth(&self) -> f32 {
        self.bandwidth
    }

    pub fn completion_time(&self) -> f32 {
        self.completion_time_ms
    }

    fn validate(&self) -> Result<(), ModelError> {
        if !self.bandwidth.is_finite() || self.bandwidth <= 0.0 {
            return Err(ModelError::InvalidBandwidth);
        }
        if !self.completion_time_ms.is_finite() || self.completion_time_ms <= 0.0 {
            return Err(ModelError::InvalidCompletionTime);
        }
        Ok(())
    }

    fn slowdown_pair(&self, baseline_bandwidth: f32, baseline_time: f32) -> (f32, f32) {
        (
            self.bandwidth / baseline_bandwidth,
            self.completion_time_ms / baseline_time,
        )
    }
}

pub fn completion_samples_to_slowdown(
    samples: &[CompletionSample],
) -> Result<Vec<(f32, f32)>, ModelError> {
    if samples.is_empty() {
        return Err(ModelError::MissingBaselineSample);
    }

    for sample in samples {
        sample.validate()?;
    }

    let baseline = samples
        .iter()
        .max_by(|a, b| {
            a.bandwidth
                .partial_cmp(&b.bandwidth)
                .unwrap_or(Ordering::Equal)
        })
        .ok_or(ModelError::MissingBaselineSample)?;

    let baseline_bandwidth = baseline.bandwidth;
    let baseline_time = baseline.completion_time_ms;

    let mut slowdown_pairs = Vec::with_capacity(samples.len());
    for sample in samples {
        slowdown_pairs.push(sample.slowdown_pair(baseline_bandwidth, baseline_time));
    }

    slowdown_pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    Ok(slowdown_pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_completion_samples_to_slowdown_pairs() -> Result<(), ModelError> {
        let samples = vec![
            CompletionSample::new(100.0, 10.0)?,
            CompletionSample::new(50.0, 15.0)?,
            CompletionSample::new(75.0, 12.5)?,
        ];

        let slowdown = completion_samples_to_slowdown(&samples)?;
        assert_eq!(slowdown.len(), 3);
        assert!((slowdown[0].0 - 0.5).abs() < 1e-6);
        assert!((slowdown[0].1 - 1.5).abs() < 1e-6);
        assert!((slowdown[2].0 - 1.0).abs() < 1e-6);
        assert!((slowdown[2].1 - 1.0).abs() < 1e-6);
        Ok(())
    }

    #[test]
    fn rejects_missing_baseline_sample() {
        let err = completion_samples_to_slowdown(&[]).unwrap_err();
        assert_eq!(err, ModelError::MissingBaselineSample);
    }

    #[test]
    fn fits_polynomial_curve() -> Result<(), ModelError> {
        let mut curve = SensitivityCurve::new(2);
        let samples: Vec<(f32, f32)> = (0..=5)
            .map(|i| {
                let bw = i as f32 * 0.2;
                let slowdown = 1.0 + 2.0 * bw + 3.0 * bw * bw;
                (bw, slowdown)
            })
            .collect();

        curve.fit(&samples)?;
        assert!((curve.coefficients[0] - 1.0).abs() < 1e-3);
        assert!((curve.coefficients[1] - 2.0).abs() < 1e-3);
        assert!((curve.coefficients[2] - 3.0).abs() < 1e-3);
        Ok(())
    }

    #[test]
    fn rejects_insufficient_curve_samples() {
        let mut curve = SensitivityCurve::new(2);
        let samples = vec![(0.0, 1.0)];
        let err = curve.fit(&samples).unwrap_err();
        assert_eq!(
            err,
            ModelError::NotEnoughSamples {
                needed: 3,
                provided: 1
            }
        );
    }

    #[test]
    fn fits_sensitivity_score() -> Result<(), ModelError> {
        let mut score = SensitivityScore { score: 0.0 };
        let samples = vec![(0.5, 1.2), (0.75, 1.8), (1.0, 1.5)];
        score.fit(&samples)?;
        assert!((score.score - 1.5).abs() < 1e-6);
        Ok(())
    }

    #[test]
    fn score_fit_rejects_empty_samples() {
        let mut score = SensitivityScore { score: 0.0 };
        let err = score.fit(&[]).unwrap_err();
        assert_eq!(err, ModelError::EmptySamples);
    }
}

/* model.rs ends here */

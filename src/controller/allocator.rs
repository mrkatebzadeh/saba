use log::debug;
use saba::model::Model;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct AppAllocation {
    pub app: String,
    pub weight: f32,
}

#[derive(Debug, Clone)]
pub struct SabaAllocator {
    iterations: usize,
    step_size: f32,
    min_share: f32,
}

impl Default for SabaAllocator {
    fn default() -> Self {
        Self {
            iterations: 200,
            step_size: 0.05,
            min_share: 0.0,
        }
    }
}

impl SabaAllocator {
    pub fn new(iterations: usize, step_size: f32, min_share: f32) -> Self {
        Self {
            iterations,
            step_size,
            min_share,
        }
    }

    pub fn allocate(&self, apps: &[(String, Model)], capacity: f32) -> Vec<AppAllocation> {
        if apps.is_empty() || capacity <= 0.0 {
            return Vec::new();
        }

        let mut weights = vec![capacity / apps.len() as f32; apps.len()];
        for _ in 0..self.iterations {
            let gradients: Vec<f32> = apps
                .iter()
                .zip(weights.iter())
                .map(|((_name, model), &bw)| model.derivative(bw))
                .collect();

            for (weight, gradient) in weights.iter_mut().zip(gradients.iter()) {
                *weight -= self.step_size * gradient;
            }

            project_to_simplex(&mut weights, capacity);
            if self.min_share > 0.0 {
                let mut updated = false;
                for weight in weights.iter_mut() {
                    if *weight < self.min_share {
                        *weight = self.min_share;
                        updated = true;
                    }
                }
                if updated {
                    project_to_simplex(&mut weights, capacity);
                }
            }
        }
        project_to_simplex(&mut weights, capacity);

        let allocations: Vec<AppAllocation> = apps
            .iter()
            .zip(weights.iter())
            .map(|((name, _), &weight)| AppAllocation {
                app: name.clone(),
                weight,
            })
            .collect();
        debug!("allocation plan: {:?}", allocations);
        allocations
    }
}

fn project_to_simplex(values: &mut [f32], target_sum: f32) {
    if values.is_empty() {
        return;
    }
    if target_sum <= 0.0 {
        values.iter_mut().for_each(|v| *v = 0.0);
        return;
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));

    let mut cumulative = 0.0;
    let mut rho = 0;
    for (idx, value) in sorted.iter().enumerate() {
        cumulative += value;
        let threshold = (cumulative - target_sum) / (idx as f32 + 1.0);
        if *value - threshold > 0.0 {
            rho = idx;
        }
    }

    let tau = (sorted.iter().take(rho + 1).sum::<f32>() - target_sum) / (rho as f32 + 1.0);
    for value in values.iter_mut() {
        *value = (*value - tau).max(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use saba::model::{Model, SensitivityCurve};

    fn curve(coefficients: Vec<f32>) -> Model {
        Model::SensitivityCurve(SensitivityCurve {
            degree_of_polynomial: coefficients.len() - 1,
            coefficients,
        })
    }

    #[test]
    fn projects_to_simplex_target_sum() {
        let mut values = vec![0.2, 0.2, 0.2];
        project_to_simplex(&mut values, 0.9);
        let sum: f32 = values.iter().sum();
        assert!((sum - 0.9).abs() < 1e-5);
    }

    #[test]
    fn allocator_prefers_sensitive_application() {
        let apps = vec![
            ("sensitive".to_string(), curve(vec![5.0, -5.0])),
            ("insensitive".to_string(), curve(vec![1.0, -0.5])),
        ];
        let allocator = SabaAllocator::default();
        let allocations = allocator.allocate(&apps, 1.0);
        let sensitive = allocations
            .iter()
            .find(|a| a.app == "sensitive")
            .unwrap()
            .weight;
        let insensitive = allocations
            .iter()
            .find(|a| a.app == "insensitive")
            .unwrap()
            .weight;
        assert!(sensitive > insensitive);
    }
}

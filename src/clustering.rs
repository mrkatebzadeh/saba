use crate::model::{Model, SensitivityCurve, SensitivityScore};
use std::error::Error;
use std::fmt;

const MAX_ITERATIONS: usize = 100;
const EPSILON: f32 = 1e-3;

#[derive(Debug, Clone, PartialEq)]
pub enum ClusteringError {
    EmptyInput,
    InvalidClusterCount,
    ZeroQueueBudget,
    Internal(&'static str),
}

impl fmt::Display for ClusteringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClusteringError::EmptyInput => write!(f, "no inputs provided for clustering"),
            ClusteringError::InvalidClusterCount => {
                write!(f, "cluster count must be greater than zero")
            }
            ClusteringError::ZeroQueueBudget => write!(f, "queue budget must be greater than zero"),
            ClusteringError::Internal(msg) => write!(f, "internal clustering error: {msg}"),
        }
    }
}

impl Error for ClusteringError {}

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationCluster {
    pub priority_level: u8,
    pub applications: Vec<String>,
    pub centroid: Model,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueueAssignment {
    pub queue_index: usize,
    pub priority_levels: Vec<u8>,
    pub centroid: Model,
}

struct FeaturePoint {
    name: String,
    params: Vec<f32>,
}

pub fn cluster_applications(
    applications: &[(String, Model)],
    desired_priority_levels: usize,
    base_priority_level: u8,
) -> Result<Vec<ApplicationCluster>, ClusteringError> {
    if applications.is_empty() {
        return Err(ClusteringError::EmptyInput);
    }
    if desired_priority_levels == 0 {
        return Err(ClusteringError::InvalidClusterCount);
    }

    let dims = applications
        .iter()
        .map(|(_, model)| model.parameters().len())
        .max()
        .unwrap_or(1);

    let features: Vec<FeaturePoint> = applications
        .iter()
        .map(|(name, model)| FeaturePoint {
            name: name.clone(),
            params: pad_parameters(&model.parameters(), dims),
        })
        .collect();

    let k = desired_priority_levels.min(features.len());
    let mut centroids = initialize_centroids(&features, k);
    let mut clusters: Vec<Vec<usize>> = vec![Vec::new(); k];

    for _ in 0..MAX_ITERATIONS {
        let mut new_clusters: Vec<Vec<usize>> = vec![Vec::new(); k];
        for (idx, point) in features.iter().enumerate() {
            let nearest = nearest_centroid(point, &centroids);
            new_clusters[nearest].push(idx);
        }
        rebalance_empty_clusters(&mut new_clusters)?;

        let mut new_centroids = centroids.clone();
        for (cluster_idx, members) in new_clusters.iter().enumerate() {
            new_centroids[cluster_idx] = mean_parameters(members, &features);
        }

        let converged = new_centroids
            .iter()
            .zip(centroids.iter())
            .all(|(a, b)| l2_distance(a, b) < EPSILON);
        centroids = new_centroids;
        clusters = new_clusters;
        if converged {
            break;
        }
    }

    let mut result = Vec::with_capacity(clusters.len());
    for (idx, members) in clusters.iter().enumerate() {
        if members.is_empty() {
            continue;
        }
        let mut apps: Vec<String> = members
            .iter()
            .map(|&member_idx| features[member_idx].name.clone())
            .collect();
        apps.sort();
        let centroid_model = model_from_parameters(centroids[idx].clone());
        result.push(ApplicationCluster {
            priority_level: base_priority_level + idx as u8,
            applications: apps,
            centroid: centroid_model,
        });
    }

    result.sort_by_key(|cluster| cluster.priority_level);
    Ok(result)
}

pub fn map_priority_levels_to_queues(
    priority_levels: &[ApplicationCluster],
    queue_budget: usize,
) -> Result<Vec<QueueAssignment>, ClusteringError> {
    if priority_levels.is_empty() {
        return Err(ClusteringError::EmptyInput);
    }
    if queue_budget == 0 {
        return Err(ClusteringError::ZeroQueueBudget);
    }

    let dims = priority_levels
        .iter()
        .map(|cluster| cluster.centroid.parameters().len())
        .max()
        .unwrap_or(1);

    let mut work: Vec<QueueCluster> = priority_levels
        .iter()
        .map(|cluster| QueueCluster {
            priority_levels: vec![cluster.priority_level],
            params_sum: pad_parameters(&cluster.centroid.parameters(), dims),
            size: 1,
        })
        .collect();

    if work.len() <= queue_budget {
        work.sort_by_key(|cluster| cluster.priority_levels[0]);
        return Ok(work
            .into_iter()
            .enumerate()
            .map(|(idx, cluster)| {
                let centroid = model_from_parameters(cluster.centroid_params());
                QueueAssignment {
                    queue_index: idx,
                    priority_levels: cluster.priority_levels,
                    centroid,
                }
            })
            .collect());
    }

    while work.len() > queue_budget {
        let mut best_pair: Option<(usize, usize, f32)> = None;
        for i in 0..work.len() {
            for j in i + 1..work.len() {
                let dist = l2_distance(&work[i].centroid_params(), &work[j].centroid_params());
                if best_pair
                    .map(|(_, _, best_dist)| dist < best_dist)
                    .unwrap_or(true)
                {
                    best_pair = Some((i, j, dist));
                }
            }
        }
        let (mut a, mut b, _) = best_pair.ok_or(ClusteringError::Internal("no pair to merge"))?;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        let right = work.remove(b);
        let left = work.remove(a);
        work.push(left.merge(right));
    }

    work.sort_by_key(|cluster| cluster.priority_levels[0]);
    Ok(work
        .into_iter()
        .enumerate()
        .map(|(queue_index, cluster)| {
            let centroid = model_from_parameters(cluster.centroid_params());
            QueueAssignment {
                queue_index,
                priority_levels: cluster.priority_levels,
                centroid,
            }
        })
        .collect())
}

fn initialize_centroids(features: &[FeaturePoint], k: usize) -> Vec<Vec<f32>> {
    let mut indices: Vec<usize> = (0..features.len()).collect();
    indices.sort_by(|a, b| features[*a].name.cmp(&features[*b].name));
    indices.truncate(k);
    indices
        .into_iter()
        .map(|idx| features[idx].params.clone())
        .collect()
}

fn nearest_centroid(point: &FeaturePoint, centroids: &[Vec<f32>]) -> usize {
    centroids
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            l2_distance(&point.params, a)
                .partial_cmp(&l2_distance(&point.params, b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}

fn mean_parameters(members: &[usize], features: &[FeaturePoint]) -> Vec<f32> {
    if members.is_empty() {
        return vec![0.0];
    }
    let dims = features[0].params.len();
    let mut sums = vec![0.0; dims];
    for &idx in members {
        for (dim, value) in features[idx].params.iter().enumerate() {
            sums[dim] += value;
        }
    }
    sums.iter_mut()
        .for_each(|value| *value /= members.len() as f32);
    sums
}

fn l2_distance(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

fn pad_parameters(params: &[f32], target: usize) -> Vec<f32> {
    let mut padded = params.to_vec();
    if padded.len() < target {
        padded.resize(target, 0.0);
    }
    padded
}

fn model_from_parameters(mut params: Vec<f32>) -> Model {
    while params.len() > 1 && params.last().map(|v| v.abs() < 1e-6).unwrap_or(false) {
        params.pop();
    }
    let degree = params.len().saturating_sub(1);
    if degree == 0 {
        return Model::SensitivityScore(SensitivityScore { score: params[0] });
    }
    Model::SensitivityCurve(SensitivityCurve {
        coefficients: params,
        degree_of_polynomial: degree,
    })
}

fn rebalance_empty_clusters(clusters: &mut [Vec<usize>]) -> Result<(), ClusteringError> {
    for idx in 0..clusters.len() {
        if clusters[idx].is_empty() {
            let (donor_idx, donor_cluster) = clusters
                .iter_mut()
                .enumerate()
                .filter(|(_, cluster)| !cluster.is_empty())
                .max_by_key(|(_, cluster)| cluster.len())
                .ok_or(ClusteringError::Internal("no donor cluster available"))?;
            let moved = donor_cluster
                .pop()
                .ok_or(ClusteringError::Internal("unable to move member"))?;
            clusters[idx].push(moved);
            if donor_idx == idx {
                return Err(ClusteringError::Internal("rebalance failed"));
            }
        }
    }
    Ok(())
}

#[derive(Clone)]
struct QueueCluster {
    priority_levels: Vec<u8>,
    params_sum: Vec<f32>,
    size: usize,
}

impl QueueCluster {
    fn centroid_params(&self) -> Vec<f32> {
        self.params_sum
            .iter()
            .map(|value| value / self.size as f32)
            .collect()
    }

    fn merge(self, other: QueueCluster) -> QueueCluster {
        let QueueCluster {
            mut priority_levels,
            params_sum,
            size,
        } = self;
        let QueueCluster {
            priority_levels: mut other_levels,
            params_sum: other_params,
            size: other_size,
        } = other;

        let summed_params: Vec<f32> = params_sum
            .iter()
            .zip(other_params.iter())
            .map(|(a, b)| a + b)
            .collect();

        priority_levels.append(&mut other_levels);
        priority_levels.sort();

        QueueCluster {
            priority_levels,
            params_sum: summed_params,
            size: size + other_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score_model(value: f32) -> Model {
        Model::SensitivityScore(SensitivityScore { score: value })
    }

    #[test]
    fn clusters_similar_applications_together() {
        let applications = vec![
            ("analytics-a".to_string(), score_model(1.0)),
            ("analytics-b".to_string(), score_model(1.1)),
            ("batch-1".to_string(), score_model(4.5)),
            ("batch-2".to_string(), score_model(4.6)),
        ];

        let clusters = cluster_applications(&applications, 2, 0).unwrap();
        assert_eq!(clusters.len(), 2);
        let low = clusters
            .iter()
            .find(|cluster| cluster.applications.contains(&"analytics-a".to_string()))
            .unwrap();
        assert!(low.applications.contains(&"analytics-b".to_string()));
        let high = clusters
            .iter()
            .find(|cluster| cluster.applications.contains(&"batch-1".to_string()))
            .unwrap();
        assert!(high.applications.contains(&"batch-2".to_string()));
    }

    #[test]
    fn maps_priority_levels_into_queue_budget() {
        let clusters = vec![
            ApplicationCluster {
                priority_level: 1,
                applications: vec!["a".into()],
                centroid: score_model(1.0),
            },
            ApplicationCluster {
                priority_level: 2,
                applications: vec!["b".into()],
                centroid: score_model(1.2),
            },
            ApplicationCluster {
                priority_level: 3,
                applications: vec!["c".into()],
                centroid: score_model(4.5),
            },
        ];

        let assignments = map_priority_levels_to_queues(&clusters, 2).unwrap();
        assert_eq!(assignments.len(), 2);
        assert_eq!(assignments[0].queue_index, 0);
        assert!(!assignments[0].priority_levels.is_empty());
        assert!(!assignments[1].priority_levels.is_empty());
        assert_eq!(
            assignments
                .iter()
                .map(|a| a.priority_levels.len())
                .sum::<usize>(),
            3
        );
    }
}

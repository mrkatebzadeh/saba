use log::debug;
use rand::seq::SliceRandom;
use saba::model::Model;
use std::cmp::Ordering;

// KMeans algorithm
fn kmeans(curves: Vec<Model>, k: usize) -> Vec<Vec<Model>> {
    let mut rng = rand::thread_rng();
    let mut centroids: Vec<Model> = Vec::new();
    let mut clusters: Vec<Vec<Model>> = vec![Vec::new(); k];

    // Initialize centroids randomly
    let mut indices: Vec<usize> = (0..curves.len()).collect();
    indices.shuffle(&mut rng);
    for i in 0..k {
        centroids.push(curves[indices[i]].clone());
    }

    loop {
        // Assign each curve to the nearest centroid
        clusters.iter_mut().for_each(|cluster| cluster.clear());

        for curve in curves.iter() {
            let nearest_centroid_index = centroids
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    curve
                        .distance(a)
                        .partial_cmp(&curve.distance(b))
                        .unwrap_or(Ordering::Equal)
                })
                .unwrap()
                .0;
            clusters[nearest_centroid_index].push(curve.clone());
        }

        // Update centroids
        let mut new_centroids: Vec<Model> = Vec::new();
        for cluster in clusters.iter() {
            if cluster.is_empty() {
                new_centroids.push(centroids[0].clone());
            } else {
                let centroid = calculate_mean(cluster);
                new_centroids.push(centroid);
            }
        }

        // Check for convergence
        let converged = centroids
            .iter()
            .zip(new_centroids.iter())
            .all(|(a, b)| a.distance(b) < 0.0001);

        if converged {
            break;
        }

        centroids = new_centroids;
    }

    clusters
}

fn calculate_mean(cluster: &Vec<Model>) -> Model {
    let mut sum = cluster[0].clone();
    for curve in cluster.iter().skip(1) {
        sum.add(curve); // Implement a method to add two models
    }
    let len = cluster.len() as f32;
    sum.divide(len) // Implement a method to divide a model by a scalar
}
//#[cfg(test)]
mod tests {
    use super::*;
    use saba::model::SensitivityCurve;
    #[test]
    fn test_kmeans() {
        let curve1 = Model::SensitivityCurve(SensitivityCurve {
            degree_of_polynomial: 1,
            coefficients: vec![1.0, 2.0, 3.0],
        });
        let curve2 = Model::SensitivityCurve(SensitivityCurve {
            degree_of_polynomial: 1,
            coefficients: vec![2.0, 3.0, 4.0],
        });
        let curve3 = Model::SensitivityCurve(SensitivityCurve {
            degree_of_polynomial: 1,
            coefficients: vec![30.0, 40.0, 50.0],
        });
        let curve4 = Model::SensitivityCurve(SensitivityCurve {
            degree_of_polynomial: 1,
            coefficients: vec![40.0, 50.0, 60.0],
        });
        let curves = vec![curve1, curve2, curve3, curve4];
        let clusters = kmeans(curves, 2);

        assert_eq!(clusters.len(), 2);
    }
}

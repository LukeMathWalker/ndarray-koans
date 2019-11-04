#[cfg(test)]
mod update_centroids_hashmap {
    use approx::assert_abs_diff_eq;
    use ndarray::{array, stack, Array, Array1, Array2, ArrayBase, Axis, Data, Ix1, Ix2};
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    // Let's re-use our incremental mean implementation
    use super::update_incremental_mean::IncrementalMean;
    use std::collections::HashMap;

    /// Iterate over our observations and capture in a HashMap the new centroids.
    /// The HashMap is a (cluster_index => new centroid) mapping.
    pub fn compute_centroids_hashmap(
        // (n_observations, n_features)
        observations: &ArrayBase<impl Data<Elem = f64>, Ix2>,
        // (n_observations,)
        cluster_memberships: &ArrayBase<impl Data<Elem = usize>, Ix1>,
    ) -> HashMap<usize, IncrementalMean> {
        let (_, n_features) = observations.dim();
        // `centroids` is a cluster index -> rolling mean mapping.
        // We will update it while we iterate over our observations.
        let mut centroids: HashMap<usize, IncrementalMean> = HashMap::new();

        // We iterate over our observations and cluster memberships in lock-step.
        let iterator = observations
            .genrows()
            .into_iter()
            .zip(cluster_memberships.iter());
        for (row, cluster_index) in iterator {
            // If we have already encountered an observation that belongs to the
            // `cluster_index`th cluster, we retrieve the current rolling mean (our new centroid)
            // and we update it using the current observation.
            if let Some(rolling_mean) = centroids.get_mut(cluster_index) {
                rolling_mean.update(&row);
            } else {
                // If we have not yet encountered an observation that belongs to the
                // `cluster_index`th cluster, we set its centroid to `row`,
                // initialising our `RollingMean` accumulator.
                let new_centroid = IncrementalMean::new(row.to_owned());
                // .to_owned takes our `row` view as input and returns an owned array.
                centroids.insert(*cluster_index, new_centroid);
            }
        }
        centroids
    }

    #[test]
    fn centroids_hashmap() {
        let cluster_size = 100;
        let n_features = 4;

        /// Let's setup a synthetic set of observations, composed of two clusters with known means
        let cluster_1: Array2<f64> =
            Array::random((cluster_size, n_features), Uniform::new(-100., 100.));
        let memberships_1 = Array1::zeros(cluster_size);
        let expected_centroid_1 = cluster_1.mean_axis(Axis(0)).unwrap();

        let cluster_2: Array2<f64> =
            Array::random((cluster_size, n_features), Uniform::new(-100., 100.));
        let memberships_2 = Array1::ones(cluster_size);
        let expected_centroid_2 = cluster_2.mean_axis(Axis(0)).unwrap();

        // `stack` combines arrays along a given axis: https://docs.rs/ndarray/0.13.0/ndarray/fn.stack.html
        let observations = stack(Axis(0), &[cluster_1.view(), cluster_2.view()]).unwrap();
        let memberships = stack(Axis(0), &[memberships_1.view(), memberships_2.view()]).unwrap();

        // Does it work?
        let centroids_hashmap = compute_centroids_hashmap(&observations, &memberships);
        assert_abs_diff_eq!(
            centroids_hashmap.get(&0).unwrap().current_mean,
            expected_centroid_1,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            centroids_hashmap.get(&1).unwrap().current_mean,
            expected_centroid_2,
            epsilon = 1e-5
        );
    }
}

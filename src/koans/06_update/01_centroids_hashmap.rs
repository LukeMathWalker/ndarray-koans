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
        __
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

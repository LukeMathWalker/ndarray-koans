#[cfg(test)]
mod update_centroids_array2 {
    use crate::path_to_enlightenment::update_centroids_hashmap::compute_centroids_hashmap;
    use approx::assert_abs_diff_eq;
    use ndarray::{array, s, stack, Array, Array1, Array2, ArrayBase, Axis, Data, Ix1, Ix2};
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    use std::collections::HashMap;

    /// As we highlighted several times, K-means is an iterative algorithm.
    /// We will perform the assignment and update steps until we are satisfied
    /// (according to a reasonable convergence criteria).
    ///
    /// If you go back to our `compute_cluster_memberships` function, the culmination of
    /// the assignment koan, you can see that it expects to receive centroids as a 2-dimensional
    /// array.
    ///
    /// Let's wrap our `compute_centroids_hashmap` to return a 2-dimensional array,
    /// where the i-th row corresponds to the i-th cluster.
    pub fn compute_centroids(
        n_centroids: usize,
        // (n_observations, n_features)
        observations: &ArrayBase<impl Data<Elem = f64>, Ix2>,
        // (n_observations,)
        cluster_memberships: &ArrayBase<impl Data<Elem = usize>, Ix1>,
    ) -> Array2<f64> {
        let centroids_hashmap = compute_centroids_hashmap(&observations, &cluster_memberships);

        // Go back to "cluster generation / dataset" if you are looking for inspiration!
        __
    }

    #[test]
    fn centroids_array2() {
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
        let centroids = compute_centroids(2, &observations, &memberships);
        assert_abs_diff_eq!(
            // `index_axis(axis, index)` returns an array with 1 less dimension,
            // taking the slice corresponding to `index` along axis `axis`.
            centroids.index_axis(Axis(0), 0),
            expected_centroid_1,
            epsilon = 1e-5
        );
        assert_abs_diff_eq!(
            // Equivalent to `centroids.index_axis(Axis(0), 1)`
            centroids.slice(s![1, ..]),
            expected_centroid_2,
            epsilon = 1e-5
        );

        assert_eq!(centroids.len_of(Axis(0)), 2);
    }
}

#[cfg(test)]
mod k_means_assembling {
    use approx::assert_abs_diff_eq;
    use ndarray::{
        array, s, stack, Array, Array1, Array2, ArrayBase, ArrayView1, ArrayView2, Axis, Data, Ix1,
        Ix2,
    };
    use ndarray_rand::rand::{Rng, SeedableRng};
    use ndarray_rand::rand_distr::StandardNormal;
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    use ndarray_stats::DeviationExt;
    use ndarray_npy::write_npy;
    use rand_isaac::Isaac64Rng;
    // All the routines we worked hard to implement!
    use super::cluster_generation_dataset::generate_dataset;
    use super::initialisation_array_base::get_random_centroids;
    use super::assignment_cluster_memberships::compute_cluster_memberships;
    use super::update_centroids_array2::compute_centroids;

    /// In the end, here we are!
    /// We just need to glue together everything we have developed so far to get a working
    /// implementation of K-means.
    ///
    /// One last obstacle to overcome: how do we decide when we have spent enough time
    /// optimizing our set of centroids and call it a day?
    ///
    /// We will use a combination of two criteria - we will stop iterating if either of the
    /// following is true:
    /// - the euclidean distance between the old set of centroids and the new set of centroids
    ///   is below `tolerance`;
    /// - the number of iteration has crossed `max_n_iterations`.
    pub fn k_means(
        n_clusters: usize,
        // (n_observations, n_features)
        observations: &ArrayBase<impl Data<Elem = f64>, Ix2>,
        rng: &mut impl Rng,
        tolerance: f64,
        max_n_iterations: usize,
    ) -> Array2<f64> {
        let mut centroids = get_random_centroids(n_clusters, observations, rng);

        let mut has_converged = false;
        let mut n_iterations = 0;

        loop {
            let memberships = compute_cluster_memberships(&centroids, observations);
            let new_centroids = compute_centroids(n_clusters, observations, &memberships);

            has_converged = __;

            centroids = new_centroids;

            if has_converged {
                break;
            }
        }

        centroids
    }

    #[test]
    fn k_means_test_drive() {
        let expected_centroids = array![[10., 10.], [1., 12.], [20., 30.], [-20., 30.],];
        let n = 1000;

        let mut rng = Isaac64Rng::seed_from_u64(42);
        let max_n_iterations = 200;
        let tolerance = 1e-5;
        let n_clusters = expected_centroids.len_of(Axis(0));

        let dataset = generate_dataset(n, expected_centroids.view(), &mut rng);

        let centroids = k_means(n_clusters, &dataset, &mut rng, tolerance, max_n_iterations);
        let cluster_memberships = compute_cluster_memberships(&centroids, &dataset);

        write_npy("python/clustered_dataset.npy", dataset)
            .expect("Failed to write .npy file");
        write_npy("python/clustered_memberships.npy", cluster_memberships.map(|&x| x as u64))
            .expect("Failed to write .npy file");
    }
}

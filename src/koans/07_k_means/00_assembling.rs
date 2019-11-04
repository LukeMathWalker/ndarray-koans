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
    // All the routines we worked hard to implement!
    // use super::cluster_generation_dataset::generate_dataset;
    use super::initialisation_array_base::get_random_centroids;
    // use super::assignment_cluster_memberships::compute_cluster_memberships;
    use super::update_centroids_array2::compute_centroids;
    use ndarray_npy::write_npy;
    use rand_isaac::Isaac64Rng;

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
            n_iterations += 1;
            let memberships = compute_cluster_memberships(&centroids, observations);
            let new_centroids = compute_centroids(observations, &memberships);

            has_converged = new_centroids.sq_l2_dist(&centroids).unwrap() < tolerance
                || n_iterations > max_n_iterations;

            centroids = new_centroids;

            if has_converged {
                break;
            }
        }

        centroids
    }

    pub fn compute_cluster_memberships(
        centroids: &ArrayBase<impl Data<Elem = f64>, Ix2>,
        observations: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    ) -> Array1<usize> {
        let cluster_memberships = observations.map_axis(Axis(1), |sample| {
            let mut iterator = centroids.genrows().into_iter();

            let first_centroid = iterator.next().expect("No centroids - degenerate case!");
            let mut closest_index = 0;
            let mut minimum_distance = sample.sq_l2_dist(&first_centroid).unwrap();

            for (index, centroid) in iterator.enumerate() {
                let distance = sample.sq_l2_dist(&centroid).unwrap();
                if distance < minimum_distance {
                    minimum_distance = distance;
                    // We skipped the first centroid in the for loop
                    closest_index = index + 1;
                }
            }

            closest_index
        });
        cluster_memberships
    }

    pub fn generate_dataset(
        cluster_size: usize,
        centroids: ArrayView2<f64>,
        rng: &mut impl Rng,
    ) -> Array2<f64> {
        // Let's allocate an array of the right shape to store the final dataset.
        // We will then progressively replace these zeros with the observations in each generated
        // cluster.
        let (n_clusters, n_features) = centroids.dim();
        let mut dataset: Array2<f64> = Array2::zeros((cluster_size * n_clusters, n_features));

        // There are many ways to iterate over an n-dimensional array.
        // `genrows` returns "generalised rows" or "lanes":
        // - regular rows of length `b`, if `self` is a 2-d array of shape `a` x `b`;
        // - `a` × `b` × ... × `l` rows each of length `m` for an n-dimensional array of shape
        //   `a` × `b` × ... × `l` × `m`.
        //
        // `enumerate` is an iterator method to get the element index in the iterator
        // alongside the element itself.
        for (cluster_index, centroid) in centroids.genrows().into_iter().enumerate() {
            let cluster = generate_cluster(cluster_size, centroid, rng);

            // Each cluster will contain `cluster_size` observations:
            // let's craft an index range in such a way that, at the end,
            // all zeros in `dataset` have been replaced with the observations in our
            // generated clusters.
            // You can create n-dimensional index ranges using the `s!` macro: check
            // the documentation for more details on its syntax and examples of this macro
            // in action - https://docs.rs/ndarray/0.13.0/ndarray/macro.s.html
            let indexes = s![
                cluster_index * cluster_size..(cluster_index + 1) * cluster_size,
                ..
            ];
            // `slice_mut` returns a **mutable view**: same principle of `ArrayView`, with the
            // privilege of mutable access.
            // As you might guess, you can only have one mutable view of an array going around
            // at any point in time.
            // The output type of `slice_mut` is `ArrayViewMut`, equivalent to `&mut [A]`
            // when comparing `Array` to `Vec`.
            dataset.slice_mut(indexes).assign(&cluster);
        }
        dataset
    }

    pub fn generate_cluster(
        n_observations: usize,
        centroid: ArrayView1<f64>,
        rng: &mut impl Rng,
    ) -> Array2<f64> {
        // `random_using` allows us to specify the random number generator we wish to use
        let n_features = centroid.len();
        let origin_cluster: Array2<f64> =
            Array::random_using((n_observations, n_features), StandardNormal, rng);
        let translation = centroid
            .broadcast((n_observations, n_features))
            .expect("Failed to broadcast");
        origin_cluster + translation
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

        write_npy("clustered_dataset.npy", dataset)
            .expect("Failed to write .npy file");
        write_npy("clustered_memberships.npy", cluster_memberships.map(|&x| x as u64))
            .expect("Failed to write .npy file");
    }
}

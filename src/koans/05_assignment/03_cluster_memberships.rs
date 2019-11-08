#[cfg(test)]
mod assignment_cluster_memberships {
    use ndarray::{array, s, Array, Array1, Array2, ArrayBase, Data, Ix2};
    use ndarray_rand::rand::SeedableRng;
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    use rand_isaac::Isaac64Rng;
    // Let's use the closest_centroid function
    use super::assignment_closest::closest_centroid;

    /// Given a matrix of centroids with shape (n_centroids, n_features)
    /// and a matrix of observations with shape (n_observations, n_features),
    /// return a 1-dimensional `membership` array such that:
    /// ```
    /// membership[i] == closest_centroid(&centroids, &observations.slice(s![i, ..])
    /// ```
    pub fn compute_cluster_memberships(
        centroids: &ArrayBase<impl Data<Elem = f64>, Ix2>,
        observations: &ArrayBase<impl Data<Elem = f64>, Ix2>,
    ) -> Array1<usize> {
        // `map_axis` returns an array with one less dimension -
        // e.g. a 1-dimensional array if applied to a 2-dimensional array.
        //
        // Each 1-dimensional slice along the specified axis is replaced with the output value
        // of the closure passed as argument.
        observations.map_axis(__, |observation| __)
    }

    #[test]
    // An observation is closest to itself.
    fn nothing_is_closer_than_self() {
        let n_centroids = 20;
        let n_features = 5;
        let mut rng = Isaac64Rng::seed_from_u64(42);
        let centroids: Array2<f64> = Array::random_using(
            (n_centroids, n_features),
            Uniform::new(-100., 100.),
            &mut rng,
        );

        let expected_memberships: Vec<usize> = (0..n_centroids).into_iter().collect();
        assert_eq!(
            compute_cluster_memberships(&centroids, &centroids),
            Array1::from(expected_memberships)
        );
    }

    #[test]
    fn oracle_test() {
        let centroids = array![[0., 0.], [1., 2.], [20., 0.], [0., 20.],];
        let observations = array![[1., 0.5], [20., 2.], [20., 0.], [7., 20.],];
        let memberships = array![0, 2, 2, 3];

        assert_eq!(
            compute_cluster_memberships(&centroids, &observations),
            memberships
        );
    }
}

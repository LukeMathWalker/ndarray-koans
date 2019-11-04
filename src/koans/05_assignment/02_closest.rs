#[cfg(test)]
mod assignment_closest {
    use approx::assert_abs_diff_eq;
    use ndarray::{array, s, Array, Array2, ArrayBase, Data, Ix1, Ix2};
    use ndarray_rand::rand::distributions::Distribution;
    use ndarray_rand::rand::SeedableRng;
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    use rand_isaac::Isaac64Rng;
    // Let's use the euclidean distance function we just wrote!
    use super::assignment_generalised_distance::euclidean_distance;

    /// Given a matrix of centroids with shape (n_centroids, n_features) and an observation,
    /// return the index of the closest centroid (the index of the corresponding row in `centroids`).
    pub fn closest_centroid(
        centroids: &ArrayBase<impl Data<Elem = f64>, Ix2>,
        observation: &ArrayBase<impl Data<Elem = f64>, Ix1>,
    ) -> usize {
        // Remember: you can use `.genrows().into_iter()` to get an iterator over the rows
        // of a 2-dimensional array.
        __
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

        let cluster_index = Uniform::new(0, n_centroids).sample(&mut rng);
        let observation = centroids.slice(s![cluster_index, ..]);

        assert_eq!(closest_centroid(&centroids, &observation), cluster_index);
    }

    #[test]
    fn oracle_test() {
        let centroids = array![[0., 0.], [1., 2.], [20., 0.], [0., 20.],];
        let observation = array![20.5, 0.5];

        assert_eq!(closest_centroid(&centroids, &observation), 2);
    }
}

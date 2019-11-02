#[cfg(test)]
mod cluster_generation_translation {
    use ndarray::{Array, Array2, Array1, array, Ix2, Axis};
    use ndarray_rand::RandomExt;
    use ndarray_rand::rand_distr::StandardNormal;
    use approx::assert_abs_diff_eq;

    pub fn generate_cluster(n_observations: usize, centroid: Array1<f64>) -> Array2<f64> {
        let n_features = centroid.dim();
        let origin_cluster: Array2<f64> = Array::random((n_observations, n_features), StandardNormal);
        let centroid_2d = centroid.into_shape((1, n_features)).unwrap();
        origin_cluster + centroid_2d
    }

    #[test]
    fn as_a_function() {
        let n_observations = 10000;
        let centroid = array![10., 10.];
        let a: Array2<f64> = generate_cluster(n_observations, centroid.clone());

        let inferred_centroid = a.mean_axis(Axis(0)).unwrap();
        let inferred_variance = a.var_axis(Axis(0), 1.);

        assert_abs_diff_eq!(inferred_centroid, centroid, epsilon = 0.1);
        assert_abs_diff_eq!(inferred_variance, array![1., 1.], epsilon = 0.1);
    }
}

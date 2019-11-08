#[cfg(test)]
mod assignment_distance {
    use approx::assert_abs_diff_eq;
    use ndarray::{array, Array, Array1, Dimension};
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;

    /// The assignment step in the K-means algorithm requires us to find the closest centroid
    /// to each observation.
    ///
    /// First things first then: let's write a function to compute the distance between
    /// two data points.
    /// We want to compute the euclidean distance:
    ///
    /// d(a, b) = sqrt[ (a₁ - b₁)² + ... + (aₙ - bₙ)²]
    ///
    /// where `a` and `b` are n-dimensional vectors.
    ///
    /// Reference: https://en.wikipedia.org/wiki/Euclidean_distance
    pub fn euclidean_distance<D: Dimension>(a: &Array<f64, D>, b: &Array<f64, D>) -> f64 {
        // No hints this time, just a bunch of tests - go ahead!
        (a - b).mapv_into(|x| x.powi(2)).sum().sqrt()
    }

    #[test]
    // Euclidean distance is symmetric.
    fn oracle_test() {
        let a = array![0., 1., 4., 2.];
        let b = array![1., 1., 2., 4.];

        assert_eq!(euclidean_distance(&a, &b), 3.);
    }

    #[test]
    // Euclidean distance is symmetric.
    fn symmetry() {
        let n_features = 100;
        let a: Array1<f64> = Array::random(n_features, Uniform::new(-100., 100.));
        let b: Array1<f64> = Array::random(n_features, Uniform::new(-100., 100.));

        assert_abs_diff_eq!(
            euclidean_distance(&a, &b),
            euclidean_distance(&b, &a),
            epsilon = 1e-5
        )
    }

    #[test]
    #[should_panic]
    // If the two arrays have different length, `euclidean_distance` should panic.
    fn invalid_input() {
        let a = array![0., 1., 2.];
        let b = array![0., 1., 2., 3.];

        euclidean_distance(&a, &b);
    }
}

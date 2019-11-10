#[cfg(test)]
mod cluster_generation_translation {
    use approx::assert_abs_diff_eq;
    use ndarray::{array, Array, Array1, Array2, Axis};
    use ndarray_npy::write_npy;
    use ndarray_rand::rand_distr::StandardNormal;
    use ndarray_rand::RandomExt;

    /// Let's take things one step further: we want to be able to specify a centroid
    /// and generate a cluster of points around it (normally distributed with unit variance,
    /// as we did before).
    pub fn generate_cluster(n_observations: usize, centroid: Array1<f64>) -> Array2<f64> {
        let n_features = __;
        let origin_cluster: Array2<f64> =
            Array::random((n_observations, n_features), StandardNormal);
        // So far we have used `Array` as one would use `Vec`: as a data structure, nothing more.
        // But `Array` is designed for numerical computations - you should not be surprised to find
        // out that `Array` implements `Add`, `Mul`, `Sub`, etc... hence you can sum, subtract
        // and element-wise multiply array together.
        //
        // There is a gotcha though: the shapes of the two operands have to be compatible.
        // You can guess as much if you uncomment the expression below and check the related
        // compiler error:
        //
        // ```
        // centroid + origin_cluster
        // ```
        //
        // `origin_cluster` has shape (n_observations, n_features) while `centroid`
        // has shape (n_features,).
        // To sum them together, we need to **view** `centroid` as a 2-d array, with the same
        // shape of `origin_cluster`.
        //
        // We can achieve this using broadcasting: we create a **view** of `centroid` that has
        // the correct shape.
        // Creating a view does not involve any copying/cloning of data or memory allocation:
        // it's equivalent to a slice for a vector - we are creating a reference to the same data
        // (or a subset of those) with a different shape information attached.
        //
        // Broadcasting is not always successful: the original shape and the final shape
        // must be compatible.
        // Check `broadcast`'s documentation for more details:
        // https://docs.rs/ndarray/0.13.0/ndarray/struct.ArrayBase.html#method.broadcast
        &centroid.broadcast(__) + &origin_cluster
        // Ndarray will also try to broadcast automatically the right operand,
        // if that is required to make the shapes of the two operands compatible.
        //
        // For example,
        //
        // ```
        // origin_cluster + centroid
        // ```
        //
        // would work without needing any explicit broadcasting, but it's useful
        // to do it manually at least once to understand what is going on
        // under the hood.
    }

    #[test]
    fn translation() {
        let n_observations = 10000;
        let centroid = array![10., 10.];
        let a: Array2<f64> = generate_cluster(n_observations, centroid.clone());

        let inferred_centroid = a.mean_axis(Axis(0)).expect("Failed to compute the mean.");
        let inferred_variance = a.var_axis(Axis(0), 1.);

        assert_abs_diff_eq!(inferred_centroid, centroid, epsilon = 0.1);
        assert_abs_diff_eq!(inferred_variance, array![1., 1.], epsilon = 0.1);

        // Use the `Cluster generation` notebook to verify that the generated
        // cluster looks like we expect it to look!
        let filename = "python/translated_cluster_smoke_check.npy";
        write_npy(filename, a).expect("Failed to write array in npy format.");
    }
}

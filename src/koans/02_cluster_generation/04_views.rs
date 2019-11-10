#[cfg(test)]
mod cluster_generation_views {
    use approx::assert_abs_diff_eq;
    use ndarray::{array, Array2, ArrayView1, Axis};

    /// Let's go for a second (or third?) take on cluster generation.
    ///
    /// In the previous example we learned about `broadcast` and how it can generate a **view**:
    /// the equivalent of an immutable slice for a `Vec`, a reference to (a subset of) the elements
    /// in an `Array`.
    ///
    /// What is a view? What is the output type of `broadcast` (once unwrapped)?
    ///
    /// It is `ArrayView`.
    /// Just as `Array`, it takes two generic parameters:
    /// - `A`, the element type;
    /// - `D`, the dimension type.
    ///
    /// `ArrayView` has read-only access to a (subset of) the data of the array it is referencing.
    /// Due to the borrow-checking rules of Rust, you can have around as many views as you want
    /// for a single array (as long as you don't have that array mutably borrowed somewhere else).
    ///
    /// Just as `Array`, there is a set of type aliases for common scenarios, e.g.
    /// `ArrayView1<A>` stands for `ArrayView<A, Ix1>`.
    ///
    /// We do not need to mutate the elements of `centroid` in `generate_cluster`.
    /// We can get away with an `ArrayView1` instead of an `Array1` as input type,
    /// thus avoiding a `.clone()` call in the test body.
    ///
    /// Can you write `generate_cluster`'s function body without peeking at the previous test?
    pub fn generate_cluster(n_observations: usize, centroid: ArrayView1<f64>) -> Array2<f64> {
        __
    }

    #[test]
    fn views() {
        let centroid = array![10., 10.];
        let a: Array2<f64> = generate_cluster(20000, __);

        let inferred_centroid = a.mean_axis(Axis(0)).expect("Failed to compute the mean.");
        let inferred_variance = a.var_axis(Axis(0), 1.);

        assert_abs_diff_eq!(inferred_centroid, centroid, epsilon = 0.1);
        assert_abs_diff_eq!(inferred_variance, array![1., 1.], epsilon = 0.1);
    }
}

#[cfg(test)]
mod cluster_generation_as_a_function {
    use ndarray::{Array, Array2, Ix2};
    use ndarray_rand::RandomExt;

    /// Let's isolate the code required to generate a cluster in a proper function,
    /// so that we can call it again from other tests.
    ///
    /// As we said before, `Array` takes two type parameters:
    /// - `A`, the element type;
    /// - `D`, the dimension type.
    ///
    /// We want to formalise in our function signature that the array we are returning
    /// has exactly two dimensions (thus allowing the compiler to verify for us **at compile-time**
    /// that we are not trying to do something non-sensical down the line, like summing
    /// arrays with different numbers of dimensions).
    ///
    /// We can use `Ix2` as dimension type, thus using `Array<f64, Ix2>` as our output type.
    /// Otherwise, we can leverage `ndarray`'s type aliases: `Array2<T>` is a shortcut
    /// for `Array<T, Ix2>`.
    /// As you can imagine, you can use `Array1`, `Array3`, etc. to work with a different number
    /// of dimensions.
    pub fn generate_cluster(n_observations: usize, n_features: usize) -> Array2<__> {
        Array::random((n_observations, n_features), __)
    }

    #[test]
    fn as_a_function() {
        let n_observations = 10000;
        let a: Array2<f64> = generate_cluster(n_observations, 2);
        let b: Array<f64, Ix2> = generate_cluster(n_observations, 3);

        assert_eq!(a.ndim(), b.ndim())
    }
}

#[cfg(test)]
mod cluster_generation_smoke_check {
    use ndarray::Array2;
    // Let's import our generation function from the previous test module
    use super::cluster_generation_as_a_function::generate_cluster;
    use ndarray_npy::{read_npy, write_npy};

    #[test]
    /// One thing is checking with a couple of assertions that mean and variance are close
    /// to what we expect.
    ///
    /// Another thing is visually confirming that the cluster we just generated has indeed
    /// that round cloudy shape that we expect it to have.
    ///
    /// `ndarray-npy` provides two convenience functions to serialize and deserialize an
    /// array in `npy` format: `read_npy` and `write_npy`.
    ///
    /// `npy` is one of the serialization format used by Python's NumPy:
    /// Rust is still quite immature when it comes to plotting, so we'll save our array in `npy`
    /// format and leverage Python to do some plotting.
    ///
    /// You can find a plug-and-play "Cluster generation" Jupyter notebook in the `python` folder,
    /// give it a go!
    fn smoke_check() {
        let a: Array2<f64> = generate_cluster(__, 2);
        let filename = "python/cluster_smoke_check.npy";

        write_npy(__, __).expect("Failed to write array in npy format.");
        let b: Array2<f64> = read_npy(__).expect("Failed to read array from npy format.");

        assert_eq!(a, b);
    }
}

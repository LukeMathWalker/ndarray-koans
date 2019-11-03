#[cfg(test)]
mod constructors_random {
    // Quite often (especially for testing purposes) you'd like to generate an array
    // filled with random values: that's where `ndarray_rand` comes in!
    //
    // `ndarray-rand` combines `ndarray` and the `rand` crate.
    // It exports `RandomExt`, an extension trait that provides additional methods
    // to generate random `Array`s - it just needs to be in scope.
    //
    // Let's give it a spin!

    use ndarray::Array;
    // Use statements to get extensions traits in scope (`QuantileExt` for `min`/`max` and
    // `RandomExt` for random array generation)
    use ndarray_rand::RandomExt;
    use ndarray_stats::QuantileExt;
    // `ndarray_rand` re-exports the `rand` and the `rand_distr` crates as submodules.
    use ndarray_rand::rand_distr::Uniform;

    #[test]
    fn random() {
        let shape = __;
        let distribution = __;
        let a = Array::random(shape, distribution);

        assert_eq!(a.ndim(), 3);
        // `min` and `max` are methods provided by `QuantileExt`, an extension trait
        // for `Array` exported by `ndarray-stats`.
        // `ndarray-stats` provides additional methods to do statistics using n-dimensional
        // arrays.
        assert!(*a.min().unwrap() >= 0);
        assert!(*a.max().unwrap() <= 10);
    }
}

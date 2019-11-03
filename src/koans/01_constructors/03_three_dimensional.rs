#[cfg(test)]
mod constructors_three_dimensional {
    use ndarray::array;

    #[test]
    fn three_dimensional() {
        // Can you guess how the `array!` macro generalises to 3-dimensional arrays?
        let tensor = array![__];

        assert_eq!(tensor.ndim(), 3);
        // `dim()` returns the shape of an n-dimensional array as a tuple
        assert_eq!(tensor.dim(), (3, 2, 2));
    }
}

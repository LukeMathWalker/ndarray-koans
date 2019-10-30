#[cfg(test)]
mod constructors_two_dimensional {
    use ndarray::{Array, array};

    #[test]
    // 1-dimensional arrays are cool, but you already knew how to do that with `Vec`.
    // You can use the `array!` macro to create multi-dimensional arrays as well!
    fn two_dimensional() {
        let matrix = array![
            [0, 1, 2],
            [3, 4, 5]
        ];

        // `.ndim()` returns the number of dimensions of an array
        assert_eq!(matrix.ndim(), __);
        assert_eq!(matrix.len(), __);
        // Indexing a multi-dimensional arrays is slightly different:
        // you need to use square brackets to specify the sequence of indexes
        // (one for each dimension of your array).
        assert_eq!(matrix[[1, 2]], __);
        assert_eq!(matrix[__], 1);
    }
}

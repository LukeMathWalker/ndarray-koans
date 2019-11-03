#[cfg(test)]
mod constructors_macro_literal {
    use ndarray::{array, Array};

    #[test]
    // You are not forced to pass through a `Vec` to create an `Array`.
    //
    // The `array!` macro follows exactly the same syntax of the `vec!` macro
    // for 1-dimensional arrays and gives you directly an `Array` instance.
    fn macro_literal() {
        let from_vector = Array::from(vec![0, 1, 2]);
        let with_macro = array![__];

        assert_eq!(from_vector, with_macro);
    }
}

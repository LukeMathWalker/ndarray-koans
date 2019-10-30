/// First things first: what is `ndarray` about?
///
/// Rust's standard library provides you with `Vec<T>`: a 1-dimensional array of elements
/// of type `T`.
///
/// Sometimes one dimension is not enough.
/// What if you want to do some linear algebra and a bunch of matrix computations?
/// What if you want to play with tensors and deep learning algorithm?
///
/// You need an n-dimensional array: here comes `ndarray`!
///
/// It provides you with `Array`, a generalisation of `Vec<T>` to handle multiple dimensions.
/// At the end of the workshop, `Array` (and `ArrayBase`) will be your new best friends.
///
/// But introductions first: how do you get your hands on one of these n-dimensional arrays?
#[cfg(test)]
mod constructors_from_vec {
    use ndarray::Array;

    #[test]
    // Given that `Array` is a generalisation of `Vec`,
    // it's fair to expect that you can get a `Vec` and turn it into an `Array`.
    fn from_vec() {
        let vector: Vec<u32> = __;

        let ndarray_vector = Array::from(vector);

        // `.len()` returns the number of elements in an array
        assert_eq!(ndarray_vector.len(), 4);
        // You can index 1-dimensional arrays using the same notation you use for `Vec`
        assert_eq!(ndarray_vector[0], 1);
        assert_eq!(ndarray_vector[2], 7);
    }
}

#[cfg(test)]
mod constructors_random_take2 {
    use ndarray::Array;
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    use ndarray_stats::QuantileExt;
    use std::any::Any;

    #[test]
    fn random() {
        // So far we have always trusted the compiler to infer the right element type for our
        // arrays based on our usage of them.
        // What if we wanted to be explicit and specify the element type?
        //
        // That's indeed possible, but we need to look a bit closer at how `Array` works.
        //
        // `Array` takes two type parameters:
        // - the element type `A`,
        // - a dimension type `D`.
        //
        // We'll get back to the dimension type `D` later.
        //
        // You can let the compiler infer either of the two using a single underscore.
        //
        // Replace the double underscores `__` appropriately to make sure that
        // `a` has elements of type `i32`.
        let a: Array<__, __> = Array::random((1000, 5), Uniform::new(1, 10));

        let element = a[[0, 0]];
        assert_eq!(element.type_id(), 0_i32.type_id())
    }
}

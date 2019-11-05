#[cfg(test)]
mod initialisation_array_base {
    use ndarray::{array, Array, Array2, ArrayBase, ArrayView1, Axis, Data, DataMut, Ix1, Ix2};
    use ndarray_rand::rand;
    use ndarray_rand::rand::{Rng, SeedableRng};
    use ndarray_rand::rand_distr::StandardNormal;
    use ndarray_rand::RandomExt;
    use rand_isaac::Isaac64Rng;

    /// [!!! Deep-dive warning - brace yourselves !!!]
    ///
    /// So far we have met three different n-dimensional array types:
    /// - `Array<A, D>`, the equivalent of `Vec<A>`. An n-dimensional array that owns its data;
    /// - `ArrayView<A, D>`, the equivalent of `&[A]`. A view on (a subset of) the data owned
    ///                      by another array;
    /// - `ArrayViewMut<A, D>`, the equivalent of `&mut [A]`. A mutable view on (a subset of) the
    ///                         data owned by another array.
    ///
    /// We have also met a bunch of type aliases: `Array1<A>` for `Array<A, Ix1>`, `ArrayView2` for
    /// `ArrayView<A, Ix2>`, etc.
    ///
    /// It turns out that `Array`, `ArrayView` and `ArrayViewMut` are type aliases too!
    /// (You might have guessed as much looking at some of the not-so-clear compiler errors you met
    /// while solving previous koans)
    ///
    /// The fundamental data structure provided by `ndarray` is `ArrayBase`.
    /// It takes two type parameters:
    /// - `S`, the data container type;
    /// - `D`, the dimensionality type.
    ///
    /// `D` accepts exactly the same dimensionality types we have seen for `Array`, `ArrayView` and
    /// `ArrayViewMut`: `Ix1`, `Ix2`, `Ix3`, etc. (plus `IxDyn` for arrays with a dynamic number
    /// of dimensions, which we will not be covering in this workshop).
    ///
    /// `S`, instead, is slightly different.
    /// It is not the element type itself, but a data **container** type which in turn accepts
    /// the element type as type parameter.
    /// `S` parametrises **ownership**: do you own your data? Can you mutate it? Can it be shared
    /// between threads?
    ///
    /// You don't have to spell `S` out explicitly (you should go for the
    /// corresponding type alias, e.g. `Array`), but it can be convenient to make `S`
    /// a generic type parameter of your function if you want to be able to accept different
    /// array types as input (e.g. useful if you are designing a public API for a crate
    /// or something as small as a single function that uses `ndarray`'s types).
    ///
    /// Let's take a second look at `get_random_centroids`: what do we need from `observations`?
    /// We don't need to mutate it.
    /// We need to index it and clone a bunch of its rows.
    /// Can we do it if `observations` is an `Array`? Yes.
    /// Can we do it if `observations` is an `ArrayView`? Yes.
    /// Can we do it if `observations` is an `ArrayViewMut`? Yes.
    ///
    /// We also do not want to consume `observations`: if it were to be an `Array`
    /// our caller would be forced to call `.clone()` before passing it to `get_random_centroids`
    /// if they need to reuse the same `Array` afterwards - wasteful, `Array`s can be huge!.
    ///
    /// We can take a reference to `ArrayBase` and constrain `S` to implement
    /// the `Data` internal trait. What is `Data`?
    /// The documentation states:
    /// ```
    /// For an array with elements that can be accessed with safe code.
    /// ```
    /// Nothing more nothing less than what we need.
    /// The container types underlying `Array`, `ArrayView` and `ArrayViewMut` all implement
    /// `Data`, hence we are good to go!
    pub fn get_random_centroids<S>(
        n_clusters: usize,
        observations: &ArrayBase<S, Ix2>,
        rng: &mut impl Rng,
    ) -> Array2<f64>
    where
        // `Data` has an associated type, `Elem`, the element type.
        // This syntax tells the compiler that `Elem` is `f64`,
        // hence we are dealing with an array of floats.
        S: Data<Elem = f64>,
    {
        let (n_samples, _) = observations.dim();
        let indices = rand::seq::index::sample(rng, n_samples, n_clusters).into_vec();
        observations.select(Axis(0), &indices)
    }

    #[test]
    fn array_base() {
        let mut rng = Isaac64Rng::seed_from_u64(42);
        let n_clusters = 3;
        let mut observations: Array2<f64> =
            Array::random_using((50, n_clusters), StandardNormal, &mut rng);

        // We can call `get_random_centroids` with a reference to all the array types
        // we have met so far - it compiles without any issue
        get_random_centroids(n_clusters, &observations, &mut rng);
        get_random_centroids(n_clusters, &observations.view(), &mut rng);
        get_random_centroids(n_clusters, &observations.view_mut(), &mut rng);
    }

    /// The other internal trait implemented by container types that
    /// you are likely to encounter sooner or later is `DataMut`:
    /// it is implemented by all container types that allow you to mutate data
    /// - e.g. `Array` and `ArrayViewMut`, but not `ArrayView`!
    pub fn double_in_place(a: &mut ArrayBase<impl DataMut<Elem = f64>, Ix1>) {
        /// `mapv_inplace` applies its closure argument to all elements in the array
        /// and replaces each entry with the closure's output.
        /// Given that it reuses the same memory locations,
        /// the closure input and output type must coincide
        a.mapv_inplace(|x| x * 2.);
    }

    #[test]
    fn mutate_array_base() {
        let mut a = array![1., 2., 3.];
        double_in_place(&mut a);
        double_in_place(&mut a.view_mut());
        // This will fail to compile!
        // Check the compiler error: can you understand what has gone wrong in light of the above context?
        // Comment it out to progress to the next exercise :)
        double_in_place(&mut a.view());
    }
}

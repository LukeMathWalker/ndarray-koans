#[cfg(test)]
mod cluster_generation_dataset {
    use ndarray::{array, s, stack, Array, Array2, ArrayView1, ArrayView2, Axis};
    use ndarray_npy::write_npy;
    use ndarray_rand::rand::{Rng, SeedableRng};
    use ndarray_rand::rand_distr::StandardNormal;
    use ndarray_rand::RandomExt;
    use rand_isaac::Isaac64Rng;

    pub fn generate_dataset(
        cluster_size: usize,
        centroids: ArrayView2<f64>,
        rng: &mut impl Rng,
    ) -> Array2<f64> {
        // Let's allocate an array of the right shape to store the final dataset.
        // We will then progressively replace these zeros with the observations in each generated
        // cluster.
        let mut dataset: Array2<f64> = Array2::zeros(__);

        // There are many ways to iterate over an n-dimensional array.
        // `genrows` returns "generalised rows" or "lanes":
        // - regular rows of length `b`, if `self` is a 2-d array of shape `a` x `b`;
        // - `a` × `b` × ... × `l` rows each of length `m` for an n-dimensional array of shape
        //   `a` × `b` × ... × `l` × `m`.
        //
        // `enumerate` is an iterator method to get the element index in the iterator
        // alongside the element itself.
        for (cluster_index, centroid) in centroids.genrows().into_iter().enumerate() {
            let cluster = generate_cluster(cluster_size, centroid, rng);

            // Each cluster will contain `cluster_size` observations:
            // let's craft an index range in such a way that, at the end,
            // all zeros in `dataset` have been replaced with the observations in our
            // generated clusters.
            // You can create n-dimensional index ranges using the `s!` macro: check
            // the documentation for more details on its syntax and examples of this macro
            // in action - https://docs.rs/ndarray/0.13.0/ndarray/macro.s.html
            let indexes = s![__];
            // `slice_mut` returns a **mutable view**: same principle of `ArrayView`, with the
            // privilege of mutable access.
            // As you might guess, you can only have one mutable view of an array going around
            // at any point in time.
            // The output type of `slice_mut` is `ArrayViewMut`, equivalent to `&mut [A]`
            // when comparing `Array` to `Vec`.
            dataset.slice_mut(indexes).assign(&cluster);
        }
        dataset
    }

    pub fn generate_cluster(
        n_observations: usize,
        centroid: ArrayView1<f64>,
        rng: &mut impl Rng,
    ) -> Array2<f64> {
        let shape = (n_observations, centroid.len());
        let origin_cluster: Array2<f64> = Array::random_using(shape, StandardNormal, rng);
        origin_cluster + centroid.broadcast(shape).expect("Failed to broadcast")
    }

    #[test]
    fn dataset() {
        // We have two choices when it comes to encoding a list of centroids:
        // - a vector of 1-dimensional arrays (`Vec<Array1<f64>>`);
        // - a 2-dimensional array (`Array2<f64>`).
        //
        // I opted for a 2-dimensional array because it encodes in the type system the fact
        // that all our centroids have the same number of features.
        // If we had used a vector of 1-dimensional arrays, we would have had to verify this
        // property at runtime.
        //
        // It's indeed worth stressing that the dimension type in `Array`, `ArrayView` and
        // `ArrayViewMut` tracks the **number** of dimensions at compile-time, but it does not
        // track the **shape** of our arrays at compile-time.
        // `array![0, 1]` and `array![0, 1, 2]` are both of type `Array1` but they have different
        // shapes, `(2,) != `(3,)`.
        let centroids = array![[10., 10.], [1., 12.], [20., 30.], [-20., 30.],];
        let n = 1000;

        let mut rng = Isaac64Rng::seed_from_u64(42);
        let dataset = generate_dataset(n, centroids.view(), &mut rng);

        assert_eq!(
            dataset.dim(),
            (centroids.shape()[0] * n, centroids.shape()[1])
        );

        // Definitely smoke check this output!
        // If all went accordingly to our plan, you should not see any observation next to (0, 0).
        // You can use again the same "Cluster generation" Jupyter notebook you used before.
        write_npy("python/dataset.npy", dataset.view()).expect("Failed to write array in npy format.");

        // There is a negligible (tiny but greater than zero) probability that our random number
        // generator genuinely spits out (0, 0).
        // But, being pragmatic, it's safe enough to assume that this assertion will only fail
        // if there is something wrong with our slicing/assignment logic.
        assert!(dataset.genrows().into_iter().all(|r| r != array![0., 0.]));
    }
}

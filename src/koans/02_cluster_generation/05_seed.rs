#[cfg(test)]
mod cluster_generation_seed {
    use ndarray::{array, Array, Array2, ArrayView1};
    use ndarray_rand::rand::{Rng, SeedableRng};
    use ndarray_rand::rand_distr::StandardNormal;
    use ndarray_rand::RandomExt;
    use rand_isaac::Isaac64Rng;

    pub fn generate_cluster(
        n_observations: usize,
        centroid: ArrayView1<f64>,
        rng: &mut impl Rng,
    ) -> Array2<f64> {
        // `random_using` allows us to specify the random number generator we wish to use
        let n_features = centroid.len();
        let origin_cluster: Array2<f64> = Array::random_using(__);
        origin_cluster + translation
    }

    #[test]
    fn seed() {
        let centroid = array![10., 10.];
        let n = 1000;

        /// We have been generating clusters quite happily so far without worrying too much
        /// about a key concept in scientific computing/ML/software in general: reproducibility!
        ///
        /// How do we make sure that the results we obtain using these synthetic clusters
        /// can be reproduced by someone else?
        ///
        /// It's a huge topic on its own, but the first **fundamental** step is making sure
        /// that the behaviour of our source of randomness is reproducible - we need to
        /// **seed** our random number generator ("rng", if you get to know each other by first name).
        ///
        /// Now, our random numbers are not **truly** random - they are pseudo-random.
        /// A pseudo-random number generator returns a sequence of values that is deterministically
        /// computed from an initial value, called **seed**.
        ///
        /// If we initialise two copies of the same pseudo random number generator
        /// using the **same** seed they should yield the same sequence of random numbers!
        ///
        /// Any random number generator that implements the `SeedableRng` trait provides
        /// a method that takes a seed as argument and returns a seeded rng.
        ///
        /// We can use `Isaac64Rng` as our seedable rng (from the `rand_isaac` crate).
        let seed = 42;
        let mut first_rng = Isaac64Rng::seed_from_u64(__);
        let mut second_rng = Isaac64Rng::seed_from_u64(__);

        let a: Array2<f64> = generate_cluster(n, centroid.view(), __);
        let b: Array2<f64> = generate_cluster(n, centroid.view(), __);

        // Will it work?
        assert_eq!(a, b);
    }
}

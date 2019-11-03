#[cfg(test)]
mod cluster_generation_seed {
    use ndarray::{Array2, array, ArrayView1, Array};
    use rand_isaac::Isaac64Rng;
    use ndarray_rand::RandomExt;
    use ndarray_rand::rand::{SeedableRng, Rng};
    use ndarray_rand::rand_distr::StandardNormal;

    pub fn generate_cluster(n_observations: usize, centroid: ArrayView1<f64>, rng: &mut impl Rng) -> Array2<f64> {
        // `random_using` allows us to specify the random number generator we wish to use
        let origin_cluster: Array2<f64> = Array::random_using(__);
        origin_cluster + centroid.broadcast((n_observations, n_features)).expect("Failed to broadcast")
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
        /// **seed** our random number generator ("rng", if you get close to each other).
        ///
        /// Now, our random numbers are not **truly** random - they are pseudo-random.
        /// A pseudo-random number generator returns a sequence of values that is deterministically
        /// computed from an initial value, called **seed**.
        ///
        /// If we initialise two copies of the same pseudo random number generator
        /// using the **same** seed they should generate the same sequence of random numbers!
        ///
        /// Any random number generator that implements the `SeedableRng` trait provides
        /// a method that takes a seed as argument and returns a seeded rng.
        ///
        /// We can use `Isaac64Rng` as our seedable generator (from the `rand_isaac` crate).
        let seed = 10;
        let mut first_rng = Isaac64Rng::seed_from_u64(seed);
        let mut second_rng = Isaac64Rng::seed_from_u64(seed);

        let a: Array2<f64> = generate_cluster(n, centroid.view(), &mut first_rng);
        let b: Array2<f64> = generate_cluster(n, centroid.view(), &mut second_rng);

        // Will it work?
        assert_eq!(a, b);
    }
}
#[cfg(test)]
mod update_incremental_mean {
    use approx::assert_abs_diff_eq;
    use ndarray::{array, Array, Array1, Array2, ArrayBase, Axis, Data, Ix1};
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;

    /// We have everything we need to perform the assignment step:
    /// given observations and centroids, we know how to assign to each observation
    /// the index of the closest cluster/centroid.
    ///
    /// The next step in the K-means algorithm is the update step:
    /// we need to re-compute the centroid (mean) of each cluster.
    ///
    /// We will solve this problem in small steps.
    /// The first task is computing the mean of a set of observations.
    ///
    /// The observation matrix will not be partitioned by cluster membership: we might have
    /// a bunch of observations belonging to the first cluster followed by one observation
    /// in the second cluster, and so on until the end of our data points.
    ///
    /// It would be convenient if we could iterate over our observations,
    /// updating the relevant new centroid one observation at a time.
    ///
    /// In other words, we want to compute **an incremental mean**.
    ///
    /// The formula to compute the new mean based on the mean of `n` previous observations and
    /// a new observation is the following:
    /// ```
    /// new_mean = current_mean + (new_observation - current_mean) / (n + 1)
    /// ```
    /// Check https://math.stackexchange.com/questions/106700/incremental-averageing for
    /// a derivation (and a nicely formatted formula).
    ///
    /// To do this successfully, we need to keep track of:
    /// - the current mean (`current_mean`);
    /// - the number of observations we have seen so far (`n`).
    ///
    /// We can store this information in a struct:

    pub struct IncrementalMean {
        pub current_mean: Array1<f64>,
        pub n_observations: usize,
    }

    impl IncrementalMean {
        pub fn new(first_observation: Array1<f64>) -> Self {
            Self {
                current_mean: first_observation,
                n_observations: 1,
            }
        }
    }

    /// We can expose a method to update the incremental mean with a new observation:

    impl IncrementalMean {
        pub fn update(&mut self, new_observation: &ArrayBase<impl Data<Elem = f64>, Ix1>) {
            // Refer to https://docs.rs/ndarray/0.13.0/ndarray/struct.ArrayBase.html#arithmetic-operations
            // when working with array arithmetic operations!
            __
        }
    }

    #[test]
    fn incremental_mean() {
        let n_observations = 100;
        let observations: Array2<f64> =
            Array::random((n_observations, 5), Uniform::new(-100., 100.));

        // We need to initialise `incremental_mean` with the first observation
        // We'll mark it as uninitialised using `None`
        let mut incremental_mean: Option<IncrementalMean> = None;

        for observation in observations.genrows().into_iter() {
            // If it has already been initialised, update it
            if let Some(mut mean) = incremental_mean.as_mut() {
                mean.update(&observation);
            // Otherwise, initialise it
            // Given that this branch is used only once, this is quite wasteful,
            // but it's easier to read... hence ¯\_(ツ)_/¯
            } else {
                // `.to_owned` takes `observation`, which has type `ArrayView1`,
                // and returns an `Array1`, performing an allocation.
                incremental_mean = Some(IncrementalMean::new(observation.to_owned()));
            }
        }

        let incremental_mean = incremental_mean.unwrap();

        assert_eq!(incremental_mean.n_observations, n_observations);
        // No significant difference between computing the mean incrementally or in a single shot
        assert_abs_diff_eq!(
            incremental_mean.current_mean,
            observations.mean_axis(Axis(0)).unwrap(),
            epsilon = 1e-5
        );
    }
}

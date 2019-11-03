#[cfg(test)]
mod initialisation_array_base {
    use ndarray::{Array, Array2, ArrayBase, ArrayView1, Data, Ix2};
    use ndarray_rand::rand::{Rng, SeedableRng};
    use ndarray_rand::rand_distr::StandardNormal;
    use ndarray_rand::RandomExt;
    use rand_isaac::Isaac64Rng;

    // K-means, as the name says, requires you to declare upfront `k`: the number of clusters you are
    // looking to spot in your dataset (quite a strong assumption to make, I agree).
    //
    // When implementing the standard K-means algorithm, the most common initialisation
    // technique is the Forgy method: as your first set of centroids, just pick `n_clusters`
    // distinct observations from your dataset - as simple as that (and it works quite well!).
    pub fn get_random_centroids(
        n_clusters: usize,
        observations: &ArrayBase<impl Data<f64>, Ix2>,
        rng: &mut impl Rng,
    ) -> Array2<f64> {
        unimplemented!()
    }

    #[test]
    fn array_base() {
        let mut rng = Isaac64Rng::seed_from_u64(42);
        let n_observations = 50;
        let n_clusters = 3;
        let observations: Array2<f64> =
            Array::random_using((n_observations, n_clusters), StandardNormal, &mut rng);

        let centroids = get_random_centroids(n_clusters, observations.view(), &mut rng);

        // Centroids are a subset of our observations:
        // each one of them corresponds to a row in `observations`.
        assert!(centroids
            .genrows()
            .into_iter()
            .all(|centroid| is_row_of(&observations, &centroid)));
    }
}

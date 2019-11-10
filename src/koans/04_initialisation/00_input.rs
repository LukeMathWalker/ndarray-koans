#[cfg(test)]
mod initialisation_input {
    use ndarray::{Array, Array2, ArrayView1};
    use ndarray_rand::rand::{Rng, SeedableRng};
    use ndarray_rand::rand_distr::StandardNormal;
    use ndarray_rand::RandomExt;
    use rand_isaac::Isaac64Rng;

    // K-means, as the name says, requires you to declare `k` upfront: the number of clusters you are
    // looking to spot in your dataset (quite a strong assumption to make, I agree).
    //
    // When implementing the standard K-means algorithm, the most common initialisation
    // technique is the Forgy method: as your first set of centroids just pick `n_clusters`
    // distinct observations from your dataset - as simple as that (and it works quite well!).
    pub fn get_random_centroids(
        n_clusters: usize,
        observations: __,
        rng: &mut impl Rng,
    ) -> Array2<f64> {
        __
    }

    // Helper function.
    // Check if there is at least one row in `matrix` that is equal to `row`
    fn is_row_of(matrix: &Array2<f64>, row: &ArrayView1<f64>) -> bool {
        matrix.genrows().into_iter().any(|r| &r == row)
    }

    #[test]
    fn input() {
        let mut rng = Isaac64Rng::seed_from_u64(42);
        let n_observations = 50;
        let n_clusters = 3;
        let n_features = 2;
        let observations: Array2<f64> =
            Array::random_using((n_observations, n_features), StandardNormal, &mut rng);

        let centroids = get_random_centroids(n_clusters, observations.view(), &mut rng);

        // Centroids are a subset of our observations:
        // each one of them corresponds to a row in `observations`.
        assert!(centroids
            .genrows()
            .into_iter()
            .all(|centroid| is_row_of(&observations, &centroid)), "Centroids should be a subset of our observations");
    }


    // Helper function nr 2.
    // Check if there is only one row in `matrix` that is equal to `row`
    fn is_unique_in(matrix: &Array2<f64>, row: &ArrayView1<f64>) -> bool {
        matrix.genrows().into_iter().filter(|r| r == row).count() == 1
    }

    #[test]
    fn test_unique_centroids() {
        let mut rng = Isaac64Rng::seed_from_u64(42);
        let n_observations = 100;
        let n_clusters = 100;
        let n_features = 3;
        let observations: Array2<f64> =
            Array::random_using((n_observations, n_features), StandardNormal, &mut rng);

        let centroids = get_random_centroids(n_clusters, observations.view(), &mut rng);

        // Each centroid should appear only once in the centroids matrix
        assert!(centroids
            .genrows()
            .into_iter()
            .all(|centroid| is_unique_in(&centroids, &centroid)), "centroids should be unique");
    }


    #[test]
    #[should_panic]
    // If the number of clusters we are looking for is bigger than the number of
    // available observations `get_random_centroids` should panic
    fn invalid_input() {
        let mut rng = Isaac64Rng::seed_from_u64(42);
        let n_observations = 4;
        let n_clusters = 5;
        let n_features = 3;
        assert!(n_observations < n_clusters);
        let observations: Array2<f64> =
            Array::random_using((n_observations, n_features), StandardNormal, &mut rng);

        get_random_centroids(n_clusters, observations.view(), &mut rng);
    }
}

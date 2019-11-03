#[cfg(test)]
mod cluster_generation_dataset {
    use ndarray::{Array2, array, ArrayView1, Array, ArrayView2, stack, Axis, s};
    use rand_isaac::Isaac64Rng;
    use ndarray_rand::RandomExt;
    use ndarray_rand::rand::{SeedableRng, Rng};
    use ndarray_rand::rand_distr::StandardNormal;
    use ndarray_npy::write_npy;

    pub fn generate_dataset(cluster_size: usize, centroids: ArrayView2<f64>, rng: &mut impl Rng) -> Array2<f64> {
        let (n_clusters, n_features) = centroids.dim();
        let dataset_shape = (n_clusters * cluster_size, n_features);
        let mut dataset: Array2<f64> = Array2::zeros(dataset_shape);

        for (i, centroid) in centroids.genrows().into_iter().enumerate() {
            let cluster = generate_cluster(cluster_size, centroid, rng);
            let indexes = s![(i * cluster_size)..((i+1) * cluster_size), ..];
            dataset.slice_mut(indexes).assign(&cluster);
        }
        dataset
    }

    pub fn generate_cluster(n_observations: usize, centroid: ArrayView1<f64>, rng: &mut impl Rng) -> Array2<f64> {
        let shape = (n_observations, centroid.len());
        let origin_cluster: Array2<f64> = Array::random_using(shape, StandardNormal, rng);
        origin_cluster + centroid.broadcast(shape).expect("Failed to broadcast")
    }

    #[test]
    fn dataset() {
        let centroids = array![
            [10., 10.],
            [1., 12.],
            [20., 30.],
            [-20., 30.],
        ];
        let n = 1000;

        let mut rng = Isaac64Rng::seed_from_u64(42);
        let dataset = generate_dataset(n, centroids.view(), &mut rng);

        // Definitely smoke check this output!
        // You can use again the "Cluster generation" Jupyter notebook.
        write_npy("dataset.npy", dataset).expect("Failed to write array in npy format.");
    }
}

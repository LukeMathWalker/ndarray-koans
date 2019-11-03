/// It took us some effort, but we now have a routine to generate a good synthetic dataset
/// to track and benchmark our efforts in clustering!
///
/// It's indeed time to start with clustering itself. How do we approach it?
///
/// K-means is an iterative algorithm: it progressively refines its choice of centroids.
/// (Quick remainder: centroid = mean of the points in a cluster)
/// It's guaranteed to converge, even though it might not find the optimal set of centroids
/// (unfortunately it can get stuck in a local minimum).
///
/// There are three steps in the K-means algorithm:
/// - initialisation step: how do we choose our initial set of centroids?
/// - assignment step: assign each observation to the nearest cluster
///                    (minimum distance between the observation and the cluster's centroid);
/// - update step: recompute the centroid (=the mean) of each cluster.
///
/// The initialisation step is a one-off, done at the very beginning.
/// Assignment and update are repeated in a loop until convergence is reached (we'll get back
/// to what this means soon enough).
///
/// We'll tackle each of these steps, one at a time, and then we'll assemble our overall
/// K-means routine combining each of the sub-pieces.
#[cfg(test)]
mod k_means_the_plan {
    #[test]
    fn the_plan() {
        let i_am_ready_to_start = __;

        assert!(i_am_ready_to_start);
    }
}

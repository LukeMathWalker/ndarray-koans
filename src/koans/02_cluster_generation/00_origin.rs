/// As specified in `greetings`, our main goal today is implementing K-means clustering.
///
/// What is it about? Wikipedia to the rescue:
///
/// ```
/// Cluster analysis or clustering is the task of grouping a set of objects in such a way
/// that objects in the same group (called a cluster) are more similar (in some sense)
/// to each other than to those in other groups (clusters).
/// ```
///
/// K-means is quite a popular algorithm when it comes to clustering: it tries to
/// partition observations into `k` clusters (`k`-means) in order to minimise the mutual
/// distance of observations belonging to the same cluster.
///
/// In mathematical terms, it tries to minimise:
///
///  k     1
///  Σ   ―――――     Σ    ‖x-y‖²
/// i=1 2*|S_i|   x,y
///              in S_i
///
/// where `S_i` is one of the `k` clusters, `x` and `y` are observations in the `S_i` cluster.
/// Check https://en.wikipedia.org/wiki/K-means_clustering#Description if you don't like
/// unicode math formulas.
///
#[cfg(test)]
mod cluster_generation_origin {
    use ndarray::{array, Array, Axis};
    use ndarray_rand::RandomExt;
    use ndarray_rand::rand_distr::StandardNormal;
    use approx::assert_abs_diff_eq;

    #[test]
    /// Our first step in our K-means implementation journey is data generation!
    ///
    /// To spot clusters, you need to have some data first.
    /// Using what we learned in the `constructors` koan, let's try to generate
    /// a bunch of 2-dimensional observations, normally distributed around the origin, (0, 0).
    fn origin_cluster() {
        let n_observations = 10000;
        let a = Array::random((n_observations, 2), StandardNormal);

        assert_abs_diff_eq!(a.mean_axis(Axis(0)).unwrap(), array![0., 0.], epsilon = 0.1);
        assert_abs_diff_eq!(a.var_axis(Axis(0), 1.), array![1., 1.], epsilon = 0.1);
    }
}

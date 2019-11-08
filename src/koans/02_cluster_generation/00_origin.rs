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
/// partition observations into `k` clusters (`k`-means) minimising the mutual
/// distance of observations belonging to the same cluster.
/// If each observation is a numerical vector, the distance is usually the euclidean distance.
///
/// In mathematical terms, it tries to minimise this loss function:
///
///  k     1
///  Σ   ―――――     Σ    ‖x-y‖²
/// i=1 2*|S_i|   x,y
///              in S_i
///
/// where `S_i` is one of the `k` clusters, `x` and `y` are observations in the `S_i` cluster.
/// Check https://en.wikipedia.org/wiki/K-means_clustering#Description if you don't like
/// unicode math formulas (rightly so).
///
#[cfg(test)]
mod cluster_generation_origin {
    use approx::assert_abs_diff_eq;
    use ndarray::{array, Array, Axis};
    use ndarray_rand::RandomExt;

    /// Our first step in our K-means implementation journey is data generation!
    ///
    /// To spot clusters, you need to have some data first.
    /// Using what we learned in the `constructors` koan, try to generate a matrix of observations:
    /// one row for each data point.
    /// We want our observations to be normally distributed around the origin, the 0 vector.
    #[test]
    fn origin_cluster() {
        let n_observations = 10000;
        let n_features = 2;
        let a = Array::random((n_observations, n_features), __);

        // The mean point of a cluster is called `centroid`.
        // We'll use this term again when implementing the actual K-means algorithm.
        // `mean_axis` can return `None` if the axis we are reducing has length 0
        // (not our case here, we can safely use `expect` or `unwrap` to get the value).
        let centroid = a.mean_axis(Axis(0)).expect("Failed to computer mean.");
        let variance = a.var_axis(Axis(0), 1.);

        // Both `mean_axis` and `var_axis` reduce the dimensionality of the array:
        // they compute the mean and the variance along the specified axis and return a
        // new array with one less dimension (the axis you specified for reduction is removed).
        assert_eq!(centroid.ndim(), __);
        assert_eq!(variance.ndim(), __);
        assert_eq!(centroid.dim(), __);
        assert_eq!(variance.dim(), __);

        // When dealing with floats, it's not a good idea to use equality checks:
        // rounding errors affect the precision of the result, making strict equality
        // quite flaky.
        // `ndarray` provides an `approx` feature-flag to bring approximate comparisons
        // according to the traits defined in the `approx` crate:
        // `assert_abs_diff_eq` checks that absolute difference between each element
        // in the two arrays is smaller than the specified `epsilon`.
        assert_abs_diff_eq!(centroid, array![0., 0.], epsilon = 0.1);
        assert_abs_diff_eq!(variance, array![1., 1.], epsilon = 0.1);

        // (Yes, we are randomly generating `a`, hence this test is not fully deterministic,
        //  but you'd have to be quite unlucky to see it fail. I cut myself some slack here.)
    }
}

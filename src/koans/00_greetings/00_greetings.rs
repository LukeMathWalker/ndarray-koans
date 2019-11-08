/// Happy RustFest!
///
/// It's my pleasure to welcome you to the "ML introduction to ndarray" workshop!
///
/// The material is structured as a series of exercises, or koans.
///
/// A koan is a riddle or puzzle that Zen Buddhists use during meditation to help them
/// unravel greater truths about the world and about themselves.
///
/// Will you get the chance to unveil deeper insights about yourself during this session?
/// Maybe, maybe not.
/// But I'll try my best to take you from "what is this ndarray thing?"
/// to "Look, ma! I can do this machine learning thing with it!".
///
/// If everything goes well, at the end of the session you will:
/// - have implemented from scratch the K-means clustering algorithm;
/// - know enough about `ndarray` and its ecosystem to go on and have fun with it!
///
/// **Practicalities**:
/// - each koan is a sub-folder in the `koans` folder;
/// - each folder contains multiple test files with a single test in each of it;
/// - you can move along your journey with `cargo run`:
///     - if you have filled in correctly a test (or you just started)
///       the console output will tell the name of the next one you should get started with;
///     - if something is wrong with your test cases, the console output will contain
///       the compiler errors or test failures that you should investigate.
///
/// ~ Enjoy! ~
///
#[cfg(test)]
mod greetings {
    #[test]
    /// This is your starting block!
    ///
    /// In each test, you are expected to replace __ in order to make test pass.
    ///
    /// Sometimes a one-liner (or a literal value) will be enough.
    /// Sometimes you will have to write a bit more to get the job done.
    ///
    /// If you get stuck, don't hesitate to ping me!
    fn the_beginning_of_your_journey() {
        let i_am_ready_to_start = true;

        assert!(i_am_ready_to_start);
    }
}

# An ML introduction to ndarray

Happy RustFest!

It's my pleasure to welcome you to the *ML introduction to ndarray* workshop!

The material is structured as a series of exercises, or koans, that you can find in the `src/koans` directory.

You can get started with
```bash
git clone git@github.com:LukeMathWalker/ndarray-koans.git
cd ndarray-koans
cargo run
```
Follow the instructions shown in the terminal to start the first exercise.

Enjoy!

## Requirements

### Software

* Rust 1.38 (or higher) with `cargo`
    * Check [link](https://www.rust-lang.org/tools/install) for installation instruction if you don't have Rust installed on your machine
    * If you already have Rust installed, run `rustc --version` to check the version.
      Run `rustup update` if you need to update your toolchain (if you installed using `rustup`)

There are some Jupyter notebooks that you will have to run to perform some data visualisations. Install
instructions for those are in `python/README.md`.

### Knowledge

A basic knowledge of Rust is assumed (the first half of the [book](https://doc.rust-lang.org/book/)?).
If you run into any issue with the language, please ping me and we'll sort it together!

## References

Throughout the workshop, the following resources might turn out to be useful:

* [Scientific Computing: A Rust adventure](https://www.lpalmieri.com/posts/2019-02-23-scientific-computing-a-rust-adventure-part-0-vectors/): an
  introduction to `ndarray` that assumes no Rust knowledge (not complete yet 😅);
* The [Rust Book](https://doc.rust-lang.org/book/);
* Docs for the crates in the `ndarray` ecosystem:
    * [`ndarray`](https://docs.rs/ndarray/0.13.0/ndarray/)
    * [`ndarray-rand`](https://docs.rs/ndarray-rand/0.11.0/ndarray_rand/)
    * [`ndarray-stats`](https://docs.rs/ndarray-stats/0.3.0/ndarray/)
    * [`ndarray-npy`](https://docs.rs/ndarray-npy/0.5.0/ndarray_npy/)
    * [`ndarray-linalg`](https://docs.rs/ndarray-linalg/0.12.0/ndarray_linalg/)
* If you are familiar with Python's `NumPy`, check [`ndarray` for `NumPy` users](https://docs.rs/ndarray/0.13.0/ndarray/doc/ndarray_for_numpy_users/index.html).



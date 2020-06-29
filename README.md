# Rust A* Algorithm

Implementation of A* pathfinding algorithm in rust. Also includes a crate that produces a wasm-binary for use in the browser.

1. `cargo test` to run the tests
1. `cargo bench` (uses [criterion](https://docs.rs/criterion/0.3.2/criterion/))
    * I need to update the test cases with some more beefy/expensive checks

This was written as a result of wanting to learn rust, and explore using it instead of js for some more performance critical code in a side project.

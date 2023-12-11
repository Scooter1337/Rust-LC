# Rust Lambda Calculus

### [L.A. (Luca) Verheul](mailto:s3704041@vuw.leidenuniv.nl)

S3704041

## Setup

- Install Rust: https://www.rust-lang.org/tools/install
- Rust version used on my machine: 1.74.1 (Not MSRV)

## Building & Running

- `cd assignment*` \
- `cargo run --release` \
  (is equal to cargo build --release && .target/release/main) \
  (the release flag is important for performance, but not for correctness, removing it adds some debug info)

## Cleaning

Just like most Makefiles implement, cargo has a clean command: `cargo clean`

## Some more interesting tools I used

- [cargo-watch](https://crates.io/crates/cargo-watch) \
  This tool is very useful for development, it watches for changes in the source code and automatically rebuilds the project.

## Benchmarking

I wouldn't be a true Rustacean if I didn't include some benchmarks.
So if I have time I will add some benchmarks.

# Rust Lambda Calculus

### [L.A. (Luca) Verheul](mailto:s3704041@vuw.leidenuniv.nl)

S3704041

## Setup

- Install Rust: https://www.rust-lang.org/tools/install
- Rust version used on my machine: 1.74.1 (Not MSRV)

## Building & Running

- `cd assignment*` \
To comply with the assignment requirements:
- `make build` \
  (is equal to cargo build --release) \
  (the release flag is important for performance, but not for correctness, removing the flag unleashes some debug macros.)
- `make run` \
  (this performs `cargo run --release`, which is normally used to build & run, but let's make it seem like we're using a legacy language.)

- By running with a filename, you can parse all rows in the file one by one
  
## Cleaning

Just like most Makefiles implement, cargo has a clean command you might want to use: `cargo clean` \
Target folders can get really large, my highest recorded size is 12GB.
For simplicity, I added it to the Makefile, so you can use `make clean` too.

## Some more interesting tools I used

- [cargo-watch](https://crates.io/crates/cargo-watch) \
  This tool is very useful for development, it watches for changes in the source code and automatically rebuilds the project.

## Benchmarking

### Assignment 1
> You can repeat these tests by running `make run -b 'a b c' 1`, `make run -b 'a b c' 1000`, `make run -b 'a b c' 1000000` \
> 

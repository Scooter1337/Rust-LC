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
- Running the program is findable in the assignment folder.

## Cleaning

Just like most Makefiles implement, cargo has a clean command you might want to use: `cargo clean` \
Target folders can get really large, my highest recorded size is 12GB.
For simplicity, I added it to the Makefile, so you can use `make clean` too.

## Some more interesting tools I used

- [cargo-watch](https://crates.io/crates/cargo-watch) \
  This tool is very useful for development, it watches for changes in the source code and automatically rebuilds the project.

## Benchmarking

### Assignment 1

> You can repeat these tests by running `make run -b 'a b c' 1`, `make run -b 'a b c' 1000`, etc.


<table class="tg">
<thead>
  <tr>
    <th class="tg-7nal"></th>
    <th class="tg-0lax abc" colspan="3">1 iteration (avg 1000 iter)</th>
    <th class="tg-0lax abc" colspan="3">1,000 iterations<br> </th>
    <th class="tg-0pky abc" colspan="3">1,000,000 iterations</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td class="tg-b4dk"></td>
    <td class="tg-ly6r">Token.</td>
    <td class="tg-ly6r">Parsing</td>
    <td class="tg-ly6r">Comb.</td>
    <td class="tg-ly6r">Token.</td>
    <td class="tg-ly6r">Parsing</td>
    <td class="tg-ly6r">Comb.</td>
    <td class="tg-ly6r">Token.</td>
    <td class="tg-ly6r">Parsing</td>
    <td class="tg-ly6r">Comb.</td>
  </tr>
  <tr>
    <td class="tg-fymr">a b c</td>
    <td class="tg-0lax">170ns</td>
    <td class="tg-0lax">410ns</td>
    <td class="tg-0lax">586ns</td>
    <td class="tg-0lax">170µs</td>
    <td class="tg-0pky">410µs</td>
    <td class="tg-0pky">586µs</td>
    <td class="tg-0pky">159ms</td>
    <td class="tg-0lax">385ms</td>
    <td class="tg-0pky">574ms</td>
  </tr>
  <tr>
    <td class="tg-fymr">(\x((a) (b)))</td>
    <td class="tg-0lax">184ns</td>
    <td class="tg-0lax">545ns</td>
    <td class="tg-0lax">755ns</td>
    <td class="tg-0lax">184µs</td>
    <td class="tg-0pky">545µs</td>
    <td class="tg-0pky">755µs</td>
    <td class="tg-0pky">180ms</td>
    <td class="tg-0lax">529ms</td>
    <td class="tg-0pky">731ms</td>
  </tr>
  <tr>
    <td class="tg-fymr">(\ x a b)</td>
    <td class="tg-0lax">171ns</td>
    <td class="tg-0lax">387ns</td>
    <td class="tg-0lax">598ns</td>
    <td class="tg-0lax">171µs</td>
    <td class="tg-0pky">387µs</td>
    <td class="tg-0pky">598µs</td>
    <td class="tg-0pky">168ms</td>
    <td class="tg-0lax">397ms</td>
    <td class="tg-0pky">606ms</td>
  </tr>
</tbody>
</table>

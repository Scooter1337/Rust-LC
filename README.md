# Rust Lambda Calculus

Lambda Calculus Assignment for Leiden University. Free choice of Programming Language.

## Assignments

### Assignment 1
#### Grade
10/10

#### Assignment Instructions

- Lambda Calculus Parser
- Precedence: lambda abstraction groups more strongly than application (i.e. abstraction precedes application), and application associates to the left.
- Grammar: ⟨expr⟩ ::= ⟨var⟩ | '(' ⟨expr⟩ ')' | '\' ⟨var⟩ ⟨expr⟩ | ⟨expr⟩ ⟨expr⟩
- See [REQUIREMENTS](<assignment 1/REQUIREMENTS.md>) for detailed requirements.

### Assignment 2
#### Grade
10/10

#### Assignment Instructions

- Lambda Calculus Interpreter (Parser + Reducer)
- Precedence: Lambda abstraction groups more strongly than application (i.e. abstraction precedes application), and application associates to the left.
- Grammar: ⟨expr⟩ ::= ⟨var⟩ | '(' ⟨expr⟩ ')' | '\' ⟨var⟩ ⟨expr⟩ | ⟨expr⟩ ⟨expr⟩
- See [REQUIREMENTS](<assignment 2/REQUIREMENTS.md>) for detailed requirements.

### Assignment 3
#### Grade
8/10 (Messed up the type checking. Didn't watch the lectures prior to doing Assignment 3, rest is perfect)

- Lambda Calculus Type Checker (Parser + Type Checker)
- Precedence: Lambda abstraction groups more strongly than application (i.e. abstraction precedes application), and application associates to the left.
- Grammar:
  - ⟨judgement⟩ ::= ⟨expr⟩ ':' ⟨type⟩
  - ⟨expr⟩ ::= ⟨lvar⟩ | '(' ⟨expr⟩ ')' | '\' ⟨lvar⟩ '^' ⟨type⟩ ⟨expr⟩ | ⟨expr⟩ ⟨expr⟩
  - ⟨type⟩ ::= ⟨uvar⟩ | '(' ⟨type⟩ ')' | ⟨type⟩ '->' ⟨type⟩ \
    lvar = variable beginning with lowercase latin letter \
    uvar = variable beginning with uppercase latin letter
- See [REQUIREMENTS](<assignment 3/REQUIREMENTS.md>) for detailed requirements.

## Setup

- Install Rust: https://www.rust-lang.org/tools/install
- Rust version used on my machine: 1.74.1 (Not MSRV)

## Building & Running

- `cd assignment*` \
  To comply with the assignment requirements:
- `make build` \
  (is equal to cargo build --release) \
  (the release flag is important for performance, but not for correctness, removing the flag unleashes some debug macros.)
- Running the program is explained in the assignment folder README.

## Cleaning

Just like most Makefiles implement, cargo has a clean command you might want to use: `cargo clean` \
Target folders can get really large, my highest recorded size is 12GB.
For simplicity, I added it to the Makefiles, so you can use `make clean` too.

## Some more interesting tools I used

- [cargo-watch](https://crates.io/crates/cargo-watch) \
  This tool is very useful for development, it watches for changes in the source code and automatically rebuilds the project.

## Benchmarking

Benched on a 3.5 GHz 12-Core Intel Xeon E5-2690V3 (24 Threads) with 64GB of DDR4-2133MT RAM. (So definitely room for single-threaded improvement.)

> You can repeat these tests by running `make run-bench EXPR="a b c" N=1000`, `make run-bench EXPR="a b c" N=1000000`, etc.

### Assignment 1

<table>
<thead>
  <tr>
    <th></th>
    <th colspan="3">1 iteration</th>
    <th colspan="3">1,000 iterations<br> </th>
    <th colspan="3">1,000,000 iterations</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td></td>
    <td>Tokenizing</td>
    <td>Parsing</td>
    <td>Combined</td>
    <td>Tokenizing</td>
    <td>Parsing</td>
    <td>Combined</td>
    <td>Tokenizing</td>
    <td>Parsing</td>
    <td>Combined</td>
  </tr>
  <tr>
    <td><code>a</code></td>
    <td>48ns</td>
    <td>49ns</td>
    <td>84ns</td>
    <td>39µs</td>
    <td>46µs</td>
    <td>83µs</td>
    <td>36ms</td>
    <td>46ms</td>
    <td>83ms</td>
  </tr>
  <tr>
    <td><code>a b c</code></td>
    <td>95ns</td>
    <td>160ns</td>
    <td>242ns</td>
    <td>80µs</td>
    <td>155µs</td>
    <td>240µs</td>
    <td>80ms</td>
    <td>159ms</td>
    <td>251ms</td>
  </tr>
  <tr>
    <td><code>(λx((a) (b)))</code></td>
    <td>152ns</td>
    <td>381ns</td>
    <td>532ns</td>
    <td>135µs</td>
    <td>305µs</td>
    <td>441µs</td>
    <td>136ms</td>
    <td>307ms</td>
    <td>445ms</td>
  </tr>
  <tr>
    <td><code>(λ x a b)</code></td>
    <td>150ns</td>
    <td>236ns</td>
    <td>362ns</td>
    <td>108µs</td>
    <td>193µs</td>
    <td>302µs</td>
    <td>102ms</td>
    <td>196ms</td>
    <td>304ms</td>
  </tr>
  <tr>
    <td><code>λx.λy.λz.a (λw.b)</code></td>
    <td>248ns</td>
    <td>569ns</td>
    <td>763ns</td>
    <td>215µs</td>
    <td>480µs</td>
    <td>704µs</td>
    <td>214ms</td>
    <td>488ms</td>
    <td>702ms</td>
  </tr>
</tbody>
</table>

### Assignment 2

<table>
<thead>
  <tr>
    <th></th>
    <th colspan="4">1,000 iterations<br> </th>
    <th colspan="4">1,000,000 iterations</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td></td>
    <td>Tokenizing</td>
    <td>Parsing</td>
    <td>Reducing</td>
    <td>Combined</td>
    <td>Tokenizing</td>
    <td>Parsing</td>
    <td>Reducing</td>
    <td>Combined</td>
  </tr>
  <tr>
    <td><code>(\x y)((\x (x x))(\x (x x)))</code></td>
    <td>338µs</td>
    <td>766µs</td>
    <td>121µs</td>
    <td>1.19ms</td>
    <td>287ms</td>
    <td>700ms</td>
    <td>113ms</td>
    <td>1.12s</td>
  </tr>
  <tr>
    <td><code>(\x x x)(\x x x)</code></td>
    <td>203µs</td>
    <td>499µs</td>
    <td>235µs</td>
    <td>926µs</td>
    <td>181ms</td>
    <td>444ms</td>
    <td>221ms</td>
    <td>870ms</td>
  </tr>
  <tr>
    <td><code>\t (\x y) t</code></td>
    <td>146µs</td>
    <td>308µs</td>
    <td>257µs</td>
    <td>691µs</td>
    <td>130ms</td>
    <td>257ms</td>
    <td>260ms</td>
    <td>660ms</td>
  </tr>
  <tr>
    <td><code>λx.λy.λz.a (λw.b)</code></td>
    <td>241µs</td>
    <td>515µs</td>
    <td>207µs</td>
    <td>951µs</td>
    <td>213ms</td>
    <td>474ms</td>
    <td>214ms</td>
    <td>918ms</td>
  </tr>
</tbody>
</table>

### Assignment 3

<table>
<thead>
  <tr>
    <th></th>
    <th colspan="4">1,000 iterations<br> </th>
    <th colspan="4">1,000,000 iterations</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td></td>
    <td>Tokenizing</td>
    <td>Parsing</td>
    <td>Type Checking</td>
    <td>Combined</td>
    <td>Tokenizing</td>
    <td>Parsing</td>
    <td>Type Checking</td>
    <td>Combined</td>
  </tr>
  <tr>
    <td><code>(\x^A (\y^(A->B) (y ((\x^A x) x)))):</code> 
        <br> 
        <code>(A -> ((A -> B) -> B))</code></td>
    <td>716µs</td>
    <td>1.47ms</td>
    <td>662µs</td>
    <td>3.5ms</td>
    <td>617ms</td>
    <td>1.40s</td>
    <td>604ms</td>
    <td>3.40s</td>
  </tr>
  <tr>
    <td><code>(\y^A (\x^(A -> (C -> A)) (x y))):</code>
    <br>
    <code>(A -> (A -> C -> A) -> C -> A)</code></td>
    <td>669µs</td>
    <td>1.33ms</td>
    <td>630µs</td>
    <td>3.51ms</td>
    <td>633ms</td>
    <td>1.31s</td>
    <td>603ms</td>
    <td>3.45s</td>
  </tr>
  <tr>
    <td><code>(\x^A x):(A -> A)</code></td>
    <td>170µs</td>
    <td>453µs</td>
    <td>280µs</td>
    <td>926µs</td>
    <td>151ms</td>
    <td>418ms</td>
    <td>258ms</td>
    <td>866ms</td>
  </tr>
  <tr>
    <td><code>(\x^B (\x^A x)):(B -> (A -> A))</code></td>
    <td>286µs</td>
    <td>741µs</td>
    <td>400µs</td>
    <td>1.47ms</td>
    <td>261ms</td>
    <td>723ms</td>
    <td>362ms</td>
    <td>1.46s</td>
  </tr>
</tbody>
</table>

# Rust Lambda Calculus Assignment 2

### [Luca]()

> OS: **MacOS Sonoma 14.2.1** \
> Arch: **ARM64** \
> Rust Compiler: **1.74.1**

Known defects: [See the full list here](REQUIREMENTS.md)

- Reduction strategy is not configurable, as my implementation for it was hacky and it didn't make sense to keep it in. Disabling alpha conversion only resulted in duplicate variables, which is not what I wanted.

# Format

- Lambda Abstraction: `λ{term}.{body}`, e.g. `λx.a b`
- Application: `{term} {term}`, e.g. `a b`
- Variable: `a`, `b`, `c`, etc. But also Unicode (only alphabetical (by choice), no emoji for example) characters, e.g. `a我`

# Reduction Strategies

The program supports the following reduction strategies:

- Beta Reduction
- Alpha Conversion

# How the program works

## Reducing

### Reduce function

1. Set step counter to 1, set varname counter to 1
2. Reduce the AST
   1. Add 1 to the step counter
   2. Match expression
      - If it is a lambda abstraction
        1. Return self but **Reduce()** body
      - If it is an application
        1. **Reduce()** left expression
        2. If resulting left expression is a lambda abstraction
           - True: Return reduction of **beta()** **reduction()** of left expression with right expression
           - False: **Reduce()** right expression, return application of left expression and right expression
      - If it is a variable, return self

### Beta Reduction function

1. match abstraction given
   - If it is a lambda abstraction
     1. **Substitute()** all occurrences of the variable in the body with the expression given, and return this resulting expression
   - If it is not a lambda abstraction, error

### Substitute function

1. match expression
   - If variable
     1. If variable is the same as the variable to substitute, return the expression to substitute
     2. If variable is not the same as the variable to substitute, return the variable
   - If application
     1. **Substitute()** left expression
     2. **Substitute()** right expression
     3. Return application of left expression and right expression
   - If lambda abstraction
     - if variable is a free variable in the body
       1. use **Alpha()** to create a new name and body for the abstraction
       2. create new abstraction using alpha name and alpha **Substitute()** alpha body
     - else return self with **Substitute()** body

### Alpha Conversion function

1. Create unique name using varname counter
2. Substitute all occurrences of the variable in the body with the unique name

### Free Variables function

Simple function that recurses to find all free variables in an expression.

## Normal / Assignment Mode

1. Read from stdin until EOF
2. Split the input into lines
3. For each line:
   1. Lex the line
      - If there is an error, print the error and exit with code 1
      - If there is no error, continue
   2. Parse the tokens
      - If there is an error, print the error and exit with code 1
      - If there is no error, reparse (as according to the requirements)
        1. Convert the expression to a string
        2. lex the string
           - If there is an error, print the error and exit with code 1
           - If there is no error, continue
        3. Parse the tokens
           - If there is an error, print the error and exit with code 1
           - If there is no error, continue
        4. Compare the outputs
           - If the output is not the same, print the error and exit with code 1
           - If the output is the same, continue to next line
   3. Reduce the AST
      - If there is an error, print the error and exit with code 1
      - If there is no error, continue
4. Print the output and exit with code 0

## Manual Mode

1. Continuously read from stdin until newline
   - If the input is `exit` or `quit`, exit with code 0
   - If the input is not `exit` or `quit`, continue
2. Lex the input
   - If there is an error, print the error and continue
   - If there is no error, continue
3. Parse the tokens
   - If there is an error, print the error and continue
   - If there is no error, continue
4. Reduce the AST
   - If there is an error, print the error and continue
   - If there is no error, print '> expression' and continue
5. GOTO 1

## Benchmark Mode

1. Get phrase and N from arguments
2. For each test
   1. Warmup
   2. Start timer
   3. Run N iterations
   4. Stop timer
3. Print times

# Usage

## Build

- `make build` or `cargo build --release`

## Running

There are 4 ways to run the program:

#### Normal Mode

- `make run < expression.txt` or `make run expression.txt`, which will read from the file `expression.txt` and exit when it is done (or errors).
- `make run`, which will read from stdin until EOF, it will not exit until you press `Ctrl+D` on Linux/MacOS or `Ctrl+Z` on Windows, and then proceed like if a file were read.

#### Manual Mode

- `make run -- -m`, or `make run-manual` which will engage Manual Mode, where you can type in expressions and press enter to immediately parse them. It will exit when you type `exit` or `quit`.

#### Benchmark Mode

- `make run-bench EXPR="{EXPR}" N={N}`, which will run the benchmark. Example usage: `make run-bench EXPR="\ x a b" N=10` or `make run-bench EXPR="a b c" N=1000`

All above commands can also be used with `cargo` instead of `make`:

- `cargo r(un) --release < expression.txt` or `cargo run --release expression.txt`
- `cargo r(un) --release`
- `cargo r(un) --release -- -m`
- `cargo r(un) --release -- -b "{EXPR}" {N}`

# Choices I made

- Non-printable ASCII characters are supported, and are treated as whitespace.

# For Rust Newbies

- All variables are immutable by default, and can be made mutable with `mut`.
- 'functions' that end with a `!` are macros, not functions. They are executed at compile time, and not at runtime. This is why `println!` is used instead of `println`.
- The last expression in a function is automatically returned, so `return` is often not needed.
- Functions, enums, etc. are imported with `use`.
- Functions, variables, macros, modules, methods are snake_case, types are UpperCamelCase.
- Rust doesn't have a `switch` statement, instead it has `match` statements.
- Rust has `iterators` that are evaluated lazily, and `collect` that collects them into a collection.
- Rust has `Ranges`, which are similar to Python's ranges, and are evaluated lazily.`
- Rust uses a Cargo.toml file to manage dependencies, profiles, etc. It is similar to a `package.json` file in Node.js.
- Instead of a garbage collector, Rust uses a concept called ['ownership'](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html). This means that every variable has an owner, and when the owner goes out of scope, the variable is dropped. This means that there is no need for a garbage collector, as the compiler can determine when a variable is no longer needed, and drop it. This is why Rust is so fast, as there is no overhead of a garbage collector. Lastly, contrary to C/C++, Rust does not have free() or malloc(), as it is not needed due to the ownership model.
- Rust has 'references', denoted with `&`. They are immutable by default, and can be made mutable with `&mut`. They are also dropped when they go out of scope.
- Instead of inheritance, Rust uses 'traits'. They are defined with the `trait` keyword, and implemented with the `impl` keyword.

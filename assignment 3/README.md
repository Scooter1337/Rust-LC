# Rust Lambda Calculus Assignment 3

### [Luca Verheul]()

> OS: **MacOS Sonoma 14.2.1** \
> Arch: **ARM64** \
> Rust Compiler: **1.75.0**

Known defects: [None](REQUIREMENTS.md)

# Format

- Lambda Abstraction: `λ{term}.{body}`, e.g. `λx.a b`
- Application: `{term} {term}`, e.g. `a b`
- Expression Variable: `a`, `b`, `c`, etc. But also Unicode (only alphabetical (by choice), no emoji for example) characters, e.g. `a我`
- Type Variable: `A`, `B`, `C`, etc. But also Unicode (only alphabetical (by choice), no emoji for example) characters, e.g. `A我`

# How the program works

Compared to ass1, the parser is only slightly modified to support judgement parsing. The lexer is also slightly modified to support Upper and Lower variables, and the new type syntax ('^', '->', ':').

## Typechecking

The typechecker walks the expression of the judgement, and collects all types, and collects free variables. If there are free variables, it errors, and tells the user which variables have no type. If there are no free variables, the typechecker will walk the judgement's type, and check if for every type variable, there is a type in the expression that matches it. If there is no match, it errors, and tells the user which type variables are unknown. If there are no errors, it will return, and the main program will print the string.

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
   3. Typecheck the AST
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
4. Typecheck the AST
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

#### Normal Mode (2 ways)

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
- Rust has `Ranges`, which are similar to Python's ranges, and are evaluated lazily.
- Rust uses a Cargo.toml file to manage dependencies, profiles, etc. It is similar to a `package.json` file in Node.js.
- Instead of a garbage collector, Rust uses a concept called ['ownership'](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html). This means that every variable has an owner, and when the owner goes out of scope, the variable is dropped. This means that there is no need for a garbage collector, as the compiler can determine when a variable is no longer needed, and drop it. This is why Rust is so fast, as there is no overhead of a garbage collector. Lastly, contrary to C/C++, Rust does not have free() or malloc(), as it is not needed due to the ownership model.
- Rust has 'references', denoted with `&`. They are immutable by default, and can be made mutable with `&mut`. They are also dropped when they go out of scope.
- Instead of inheritance, Rust uses 'traits'. They are defined with the `trait` keyword, and implemented with the `impl` keyword.

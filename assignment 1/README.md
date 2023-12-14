# Format

- Lambda: `λ{term}.{body}`, e.g. `λx.a b`
- Application: `{term} {term}`, e.g. `a b`
- Variable: `a`, `b`, `c`, etc. But also Unicode (only alphabetical (by choice), no emoji for example) characters, e.g. `a我`

# How the program works

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
           - If the output is the same, continue to next line
           - If the output is not the same, print the error and exit with code 1
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
   - If there is no error, print `> expression`

## Benchmark Mode

1. Get phrase and N from arguments
2. For each test
   1. Warmup 1,000,000 iterations
   2. Start timer
   3. Run N iterations
   4. Stop timer
3. Print times

# Usage

## Build

- `make build` or `cargo build --release`

## Running

There are 4 ways to run the program:

- `make run < expression.txt` or `make run expression.txt`, which will read from the file `expression.txt` and exit when it is done (or errors).
- `make run`, which will read from stdin until EOF, it will not exit until you press `Ctrl+D` on Linux/MacOS or `Ctrl+Z` on Windows, and then proceed like if a file were read.
- `make run -- -m`, or `make run-manual` which will engage Manual Mode, where you can type in expressions and press enter to immediately parse them. It will exit when you type `exit` or `quit`.
- `make run-bench EXPR="{EXPR}" N={N}`, which will run the benchmark. Example usage: `make run-bench EXPR="\ x a b" N=10` or `make run-bench EXPR="a b c" N=1000`

# Choices I made

- Variable names can (by choice) only contain alphanumeric characters (in any language, unicode), it did not make sense to add support for other characters like emoji, etc.
- Instead of using Rust's built-in 'Panic', I manually return an error message and exit the program with an exit code. This is to comply with the requirements of the assignment, as panic exits with 101, and not with the required 1.

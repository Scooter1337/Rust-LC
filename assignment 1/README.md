- Variable names can (by choice) only contain alphanumeric characters (in any language, unicode), it did not make sense to add support for other characters like emoji, etc.
- Instead of using Rust's built-in 'Panic', I manually retturn an error message and exit the program with an exit code. This is to comply with the requirements of the assignment, as panic exits with 101, and not with 1.

# Usage

## Build

- `make build`

## Running

There are 4 ways to run the program:

- `make run`, which will read from stdin until EOF, it will not exit until you press `Ctrl+D` on Linux/MacOS or `Ctrl+Z` on Windows.
- `make run < expression.txt` or `make run expression.txt`, which will read from the file `expression.txt` and exit when it is done (or errors).
- `make run -m`, or `make run-manual` which will engage Manual Mode, where you can type in expressions and press enter to parse them. It will exit when you type `exit` or `quit`.
- `make run -b "{expr}" {times}`, which will run the benchmark. Example usage: `make run -b "a b c" 1000` or `make run-bench EXPR="a b c" N=1000`

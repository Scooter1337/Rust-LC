// L.A. (Luca) Verheul - S3704041
// Mon 11 Dec 2023

mod line_reader;
use line_reader::{read_lines_from_file, read_lines_from_terminal};

mod tokenizer;
use reducer::reduce;
use tokenizer::tokenize;

mod parser;
use parser::{parse, Expression};

mod utils;

mod bench;
use bench::bench;

mod manual_mode;
use manual_mode::manual_mode;

#[allow(dead_code, unused)]
mod reducer;

#[allow(dead_code, unused)]
fn main() {
    // if args provided
    let lines: Vec<String>;
    if std::env::args().len() > 1 {
        // get args
        let args = std::env::args().collect::<Vec<String>>();

        match args[1].as_str() {
            "-m" => {
                manual_mode();
                return;
            }
            "-b" => {
                bench(args);
                return;
            }
            arg => {
                lines = read_lines_from_file(arg);
            }
        }

        if lines.is_empty() {
            panic!("Empty file!");
        }
    } else {
        // read input from terminal
        lines = read_lines_from_terminal();
    }

    if lines.len() > 1 {
        eprintln!("Warning: multiple lines provided, but as per assignment spec: 'The program should accept only one expression in the input file.' Shutdown imminent!");
        std::process::exit(1);
    }

    let expressions: Vec<Expression> = lines
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            let tokens = tokenize(&line, idx);
            let expression = parse(&tokens, idx);
            // convert the expression to a string, making use of the Display trait
            let exprstring = expression.to_string();

            // reparse the expression
            let tokens2 = tokenize(&exprstring, idx);
            let expression2 = parse(&tokens2, idx);

            // check if the expressions are equal
            let expression = match expression == expression2 {
                true => expression2,
                false => {
                    eprintln!(
                        "Invalid expression (on reparse) '{}' is not equal to '{}' on line {}!",
                        expression2,
                        expression,
                        idx + 1
                    );
                    std::process::exit(1);
                }
            };

            reduce(expression, idx)
        })
        .collect();
    // We can only get here if we have 0 errors, so print the expressions
    for expr in expressions {
        println!("{}", expr);
    }
}

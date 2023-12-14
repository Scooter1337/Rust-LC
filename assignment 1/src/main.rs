// L.A. (Luca) Verheul - S3704041
// Mon 11 Dec 2023

mod line_reader;
use line_reader::{read_lines_from_file, read_lines_from_terminal};

mod tokenizer;
use tokenizer::tokenize;

mod parser;
use parser::{parse, Expression};

mod utils;

mod bench;
use bench::bench;

mod manual_mode;
use manual_mode::manual_mode;

#[allow(dead_code, unused)]
fn main() {
    // if args provided, read from file
    let lines: Vec<String>;
    if std::env::args().len() > 1 {
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
    }
    // continuously read input from terminal
    else {
        lines = read_lines_from_terminal();
    }

    let expressions: Vec<Expression> = lines
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            // tokenize each line
            let tokens = tokenize(&line, Some(idx));
            let expression = parse(&tokens, Some(idx));
            let exprstring = expression.to_string();

            // reparse the expression
            let tokens2 = tokenize(&exprstring, Some(idx));
            let expression2 = parse(&tokens2, Some(idx));

            // check if the expressions are equal
            match expression == expression2 {
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
            }
        })
        .collect();
    // Will have panicked if invalid expression
    for expr in expressions {
        println!("{}", expr);
    }
}

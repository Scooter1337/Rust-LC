// L.A. (Luca) Verheul - S3704041
// Wo 13 Dec 2023

#[allow(dead_code)]
mod line_reader;
use line_reader::*;

#[allow(dead_code)]
mod tokenizer;
use tokenizer::*;

#[allow(dead_code)]
mod parser;
use parser::*;

#[allow(dead_code)]
mod utils;

#[allow(dead_code)]
mod bench;
use bench::*;

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
            let tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize();
            dbg!(&tokens);
            match tokens {
                Err(err_code) => {
                    eprintln!(
                        "Invalid expression '{}' caught during tokenizing on line {}!",
                        err_code,
                        idx + 1
                    );
                    std::process::exit(1);
                }
                Ok(tokens) => {
                    let expression = parser::parse(&tokens);
                    dbg!(&expression);
                    match expression {
                        Ok(expression) => expression,
                        Err(err_code) => {
                            eprintln!(
                                "Invalid expression '{}' caught during parsing on line {}!",
                                err_code,
                                idx + 1
                            );
                            std::process::exit(1);
                        }
                    }
                }
            }
        })
        .collect();
    // Will have panicked if invalid expression
    for expr in expressions {
        println!("{}", expr);
    }
}

fn manual_mode() {
    println!("Manual mode activated!");
    println!("Enter an expression to parse it.");
    loop {
        print!("> ");
        let input = read_line_from_terminal();
        let tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        dbg!(&tokens);
        match tokens {
            Err(err_code) => {
                eprintln!("Invalid expression '{}'!", err_code);
                continue;
            }
            Ok(tokens) => {
                let expression = parser::parse(&tokens);
                dbg!(&expression);
                match expression {
                    Ok(expression) => println!("{}", expression),
                    Err(err_code) => {
                        eprintln!("Invalid expression '{}'!", err_code);
                        continue;
                    }
                }
            }
        }
    }
}

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

#[allow(dead_code)]
mod utils;

#[allow(dead_code, unused)]
fn main() {
    // if args provided, read from file
    if std::env::args().len() > 1 {
        // read lines from file
        let lines = read_lines_from_file();

        if lines.is_empty() {
            panic!("Empty file!");
        }

        lines.into_iter().for_each(|line| {
            // tokenize each line
            let tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize();
            dbg!(&tokens);
            if let Ok(tokens) = tokens {
                let expression = parser::parse(&tokens).unwrap();
                dbg!(&expression);
                println!("{}", expression);
            } else {
                println!("Invalid expression!");
            }
        });
    }
    // continuously read input from terminal
    else {
        println!("Welcome to the lambda calculus interpreter!");
        println!("Enter a lambda expression to evaluate:");
        // loop {
        if let Ok(line) = read_line_from_terminal() {
            if line.is_empty() {
                // continue;
            }
            dbg!(&line);
            // tokenize line
            let tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize();
            dbg!(&tokens);
            if let Ok(tokens) = tokens {
                let expression = parser::parse(&tokens).unwrap();
                dbg!(&expression);
                println!("{}", expression);
            }
        } else {
            println!("Invalid expression!");
        }
        // }
    }
}

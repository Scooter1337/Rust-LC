// L.A. (Luca) Verheul - S3704041
// Wed 14 Dec 2023

use std::io::{self, Write};

use crate::{
    line_reader::read_line_from_terminal, parser::manual_parse, tokenizer::manual_tokenize,
};

pub(super) fn manual_mode() {
    println!("Manual mode activated!");
    println!("Enter an expression to parse it.");
    loop {
        print!("Expression: ");
        io::stdout().flush().unwrap();
        let input = read_line_from_terminal();
        let tokens = manual_tokenize(&input);
        if tokens.is_none() {
            continue;
        }
        let expression = manual_parse(&tokens.unwrap());
        if expression.is_none() {
            continue;
        }
        let exprstring = expression.unwrap().to_string();
        let tokens2 = manual_tokenize(&exprstring);
        if tokens2.is_none() {
            continue;
        }
        let expression2 = manual_parse(&tokens2.unwrap());
        if expression2.is_none() {
            continue;
        }
        let exprstring2 = expression2.unwrap().to_string();
        if exprstring == exprstring2 {
            println!("> {exprstring}");
        } else {
            println!("> {exprstring} != {exprstring2}");
            println!("First Parse: {}", exprstring);
            println!("Reparse: {}", exprstring2);
        }
    }
}

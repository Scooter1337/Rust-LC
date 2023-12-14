// L.A. (Luca) Verheul - S3704041
// Wed 14 Dec 2023

use crate::{line_reader::read_line_from_terminal, parser::parse, tokenizer::tokenize};

pub(super) fn manual_mode() {
    println!("Manual mode activated!");
    println!("Enter an expression to parse it.");
    loop {
        let input = read_line_from_terminal();
        let tokens = tokenize(&input, None);
        dbg!(&tokens);
        let expression = parse(&tokens, None);
        println!("> {}", expression);
    }
}

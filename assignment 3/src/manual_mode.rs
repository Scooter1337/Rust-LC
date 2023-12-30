// L.A. (Luca) Verheul - S3704041
// Wed 14 Dec 2023

use std::io::{self, Write};

use crate::{
    line_reader::read_line_from_terminal, parser::manual_parse, tokenizer::manual_tokenize,
    type_checker::manual_type_check,
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
        let judgement = manual_parse(&tokens.unwrap());
        if judgement.is_none() {
            continue;
        }
        let jdgmtstring = judgement.unwrap().to_string();
        let tokens2 = manual_tokenize(&jdgmtstring);
        if tokens2.is_none() {
            continue;
        }
        let judgement2 = manual_parse(&tokens2.unwrap());
        if judgement2.is_none() {
            continue;
        }
        let jdgmtstring2 = judgement2.clone().unwrap().to_string();
        if jdgmtstring == jdgmtstring2 {
            if manual_type_check(&judgement2.unwrap()) {
                println!("> {jdgmtstring}");
            }
        } else {
            println!("> {jdgmtstring} != {jdgmtstring2}");
            println!("First Parse: {}", jdgmtstring);
            println!("Reparse: {}", jdgmtstring2);
        }
    }
}

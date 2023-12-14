// L.A. (Luca) Verheul - S3704041
// Mon 11 Dec 2023

// Import handy dbg! macro (shadowing std::dbg! macro)
use crate::dbg;

use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

/// Read lines from file
/// Standard mode: read until EOF
pub(super) fn read_lines_from_file(filename: &str) -> Vec<String> {
    // get os args input

    let mut lines = Vec::new();
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file: {}", error);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.expect("Could not read line"));
    }

    dbg!(&lines);
    lines
}

/// Read lines from terminal
/// Standard mode: read until EOF
pub(super) fn read_lines_from_terminal() -> Vec<String> {
    // vec for storing lines
    let mut lines = Vec::new();

    // lock stdin and get lines
    let input = stdin().lock().lines();
    for line in input {
        match line {
            Ok(line) => {
                lines.push(line);
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                std::process::exit(1);
            }
        }
    }
    lines
}

/// Read line from terminal
/// REPL mode: read until newline
pub(super) fn read_line_from_terminal() -> String {
    let mut input = String::new();
    match stdin().lock().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim().to_string();
            dbg!(&input);
            if input == "quit" || input == "exit" {
                std::process::exit(0);
            }
            input
        }
        Err(error) => {
            eprintln!("Error reading line: {}", error);
            std::process::exit(1);
        }
    }
}

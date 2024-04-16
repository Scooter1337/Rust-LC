// L.A. (Luca) Verheul - S3704041
// Mon 11 Dec 2023

mod line_reader;
use line_reader::{read_lines_from_file, read_lines_from_terminal};

mod tokenizer;
use tokenizer::tokenize;

mod parser;
use parser::{parse, Judgement};

mod type_checker;
use type_checker::type_check;

mod utils;

mod bench;
use bench::bench;

mod manual_mode;
use manual_mode::manual_mode;

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

    let judgements: Vec<Judgement> = lines
        .into_iter()
        .enumerate()
        .map(|(idx, line)| {
            let tokens = tokenize(&line, idx);
            let judgement = parse(&tokens, idx);
            // convert the judgement to a string, making use of the Display trait
            let judgement_string = judgement.to_string();
            dbg!(&judgement_string);

            // reparse the judgement
            let tokens2 = tokenize(&judgement_string, idx);
            let judgement2 = parse(&tokens2, idx);

            // check if the judgements are equal
            if judgement_string != judgement2.to_string() {
                eprintln!(
                    "Invalid judgement (on second parse) '{}' is not equal to '{}' on line {}!",
                    judgement2,
                    judgement,
                    idx + 1
                );
                std::process::exit(1);
            }

            // type check the judgement
            type_check(&judgement, idx);
            judgement
        })
        .collect();
    // We can only get here if we have 0 errors, so print the judgements
    for judgement in judgements {
        println!("{}", judgement);
    }
}

#[allow(dead_code)]
mod line_reader;
#[allow(dead_code)]
mod tokenizer;

#[allow(dead_code)]
mod parser;

#[allow(dead_code, unused)]
fn main() {
    // if no args provided
    if std::env::args().len() > 1 {
        // read lines from file
        let lines = line_reader::read_lines_from_file();

        if lines.is_empty() {
            panic!("Empty file!");
        }

        lines.iter().for_each(|line| {
            // tokenize each line
            let tokenizer = tokenizer::Tokenizer::new(line.to_string());
            let tokens = tokenizer.tokenize();
        });
    }
    // continuously read input from terminal
    else {
        println!("Welcome to the lambda calculus interpreter!");
        println!("Enter a lambda expression to evaluate:");
        loop {
            if let Ok(line) = line_reader::read_line_from_terminal() {
                dbg!(line);
            }
        }
    }
}

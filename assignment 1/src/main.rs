mod file_reader;
mod tokenizer;

fn main() {
    // if no args provided
    if std::env::args().len() < 2 {
        panic!("Usage: cargo run (--release) <filename>");
    }

    // read lines from file
    let lines = file_reader::read_lines_from_file();
}

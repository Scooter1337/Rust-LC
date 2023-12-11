use std::{
    env::args,
    fs::File,
    io::{stdin, BufRead, BufReader, Error},
};

pub(super) fn read_lines_from_file() -> Vec<String> {
    // get os args input
    let args: Vec<String> = args().collect();

    // get filename from args
    let filename = &args[1];

    let mut lines = Vec::new();
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.expect("Could not read line"));
    }
    dbg!(&lines);
    lines
}

pub(super) fn read_line_from_terminal() -> Result<String, Error> {
    let mut input = String::new();
    match stdin().lock().read_line(&mut input) {
        Ok(_) => {
            // process the input here
            let trimmed_input = input.trim();
            dbg!(trimmed_input);
            Ok(trimmed_input.to_string())
        }
        Err(error) => {
            println!("Error reading input: {}", error);
            Err(error)
        }
    }
}

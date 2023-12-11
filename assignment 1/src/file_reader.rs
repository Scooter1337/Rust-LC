use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader},
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
    dbg!("Readlines: {:?}", &lines);
    lines
}

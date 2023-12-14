// L.A. (Luca) Verheul - S3704041
// Wed 13 Dec 2023

use crate::parser::bench_parse;
use crate::tokenizer::bench_tokenize;

pub fn bench(args: Vec<String>) {
    let phrase = args.get(2).expect("No expression given!").clone();
    let times = args
        .get(3)
        .expect("No iterations given!")
        .parse::<usize>()
        .unwrap();

    // warmup
    println!("Warming up...");
    for _ in 0..1000000 {
        bench_tokenize(&phrase).unwrap();
    }
    println!("Done warming up.");

    let now1 = std::time::Instant::now();
    for _ in 0..times {
        bench_tokenize(&phrase).unwrap();
    }
    let elapsed1 = now1.elapsed();

    let tokens = bench_tokenize(&phrase).unwrap();

    // warmup
    println!("Warming up...");
    for _ in 0..1000000 {
        bench_parse(&tokens).unwrap();
    }
    println!("Done warming up.");

    let now2 = std::time::Instant::now();
    for _ in 0..times {
        bench_parse(&tokens).unwrap();
    }
    let elapsed2 = now2.elapsed();

    // warmup
    println!("Warming up...");
    for _ in 0..1000000 {
        bench_tokenize(&phrase).unwrap();
        bench_parse(&tokens).unwrap();
    }
    println!("Done warming up.");

    let now3 = std::time::Instant::now();
    for _ in 0..times {
        bench_tokenize(&phrase).unwrap();
        bench_parse(&tokens).unwrap();
    }
    let elapsed3 = now3.elapsed();
    println!("Tokenizing {} {} times took {:?}", phrase, times, elapsed1);
    println!("Parsing {} {} times took {:?}", phrase, times, elapsed2);
    println!(
        "Tokenizing and parsing (combined) {} {} times took {:?}",
        phrase, times, elapsed3
    );
}

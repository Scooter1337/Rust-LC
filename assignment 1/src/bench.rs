use crate::parser;
use crate::tokenizer::*;

pub fn bench(args: Vec<String>) {
    let phrase = args[2].clone();
    let times = args[3].parse::<usize>().unwrap();
    let tokenizer = Tokenizer::new(phrase.clone());

    // warmup
    println!("Warming up...");
    for _ in 0..1000000 {
        tokenizer.tokenize().unwrap();
    }
    println!("Done warming up.");

    let now1 = std::time::Instant::now();
    for _ in 0..times {
        tokenizer.tokenize().unwrap();
    }
    let elapsed1 = now1.elapsed();

    let tokens = tokenizer.tokenize().unwrap();

    // warmup
    println!("Warming up...");
    for _ in 0..1000000 {
        parser::parse(&tokens).unwrap();
    }
    println!("Done warming up.");

    let now2 = std::time::Instant::now();
    for _ in 0..times {
        parser::parse(&tokens).unwrap();
    }
    let elapsed2 = now2.elapsed();

    // warmup
    println!("Warming up...");
    for _ in 0..1000000 {
        tokenizer.tokenize().unwrap();
        parser::parse(&tokens).unwrap();
    }
    println!("Done warming up.");

    let now3 = std::time::Instant::now();
    for _ in 0..times {
        tokenizer.tokenize().unwrap();
        parser::parse(&tokens).unwrap();
    }
    let elapsed3 = now3.elapsed();
    println!("Tokenizing {} {} times took {:?}", phrase, times, elapsed1);
    println!("Parsing {} {} times took {:?}", phrase, times, elapsed2);
    println!(
        "Tokenizing and parsing (combined) {} {} times took {:?}",
        phrase, times, elapsed3
    );
}

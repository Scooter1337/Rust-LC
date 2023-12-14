// L.A. (Luca) Verheul - S3704041
// Wed 13 Dec 2023

use crate::parser::bench_parse;
use crate::tokenizer::bench_tokenize;

pub(super) fn bench(args: Vec<String>) {
    let expression = args.get(2).expect("No expression given!").clone();
    let times = args
        .get(3)
        .expect("No iterations given!")
        .parse::<usize>()
        .unwrap();

    // <Test tokenize>
    //
    // <warmup>
    println!("Warming up...");
    for _ in 0..1000000 {
        bench_tokenize(&expression);
    }
    println!("Done warming up.");
    // <test>
    let now1 = std::time::Instant::now();
    for _ in 0..times {
        bench_tokenize(&expression);
    }
    let elapsed1 = now1.elapsed();
    // </Test tokenize>

    let tokens = bench_tokenize(&expression);

    // <Test parse>
    //
    // <warmup>
    println!("Warming up...");
    for _ in 0..1000000 {
        bench_parse(&tokens);
    }
    println!("Done warming up.");
    // <test>
    let now2 = std::time::Instant::now();
    for _ in 0..times {
        bench_parse(&tokens);
    }
    let elapsed2 = now2.elapsed();
    // </Test parse>

    // <Test combined>
    //
    // <warmup>
    println!("Warming up...");
    for _ in 0..1000000 {
        bench_tokenize(&expression);
        bench_parse(&tokens);
    }
    println!("Done warming up.");
    // <test>
    let now3 = std::time::Instant::now();
    for _ in 0..times {
        bench_tokenize(&expression);
        bench_parse(&tokens);
    }
    let elapsed3 = now3.elapsed();
    // </Test combined>

    // print results
    println!(
        "Tokenizing {} {} times took {:?}",
        expression, times, elapsed1
    );
    println!("Parsing {} {} times took {:?}", expression, times, elapsed2);
    println!(
        "Tokenizing and parsing (combined) {} {} times took {:?}",
        expression, times, elapsed3
    );
}

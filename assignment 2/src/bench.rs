// L.A. (Luca) Verheul - S3704041
// Wed 13 Dec 2023

use crate::parser::bench_parse;
use crate::reducer::bench_reduce;
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

    // <Test combined including reduce>
    //
    // <warmup>
    println!("Warming up...");
    for _ in 0..1000000 {
        bench_reduce(bench_parse(&bench_tokenize(&expression)));
    }
    println!("Done warming up.");
    // <test>
    let now4 = std::time::Instant::now();
    for _ in 0..times {
        bench_reduce(bench_parse(&bench_tokenize(&expression)));
    }
    let elapsed4 = now4.elapsed();
    // </Test combined including reduce>

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
    println!(
        "Tokenizing, parsing and reducing (combined) {} {} times took {:?}",
        expression, times, elapsed4
    );
}

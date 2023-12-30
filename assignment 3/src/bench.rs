// L.A. (Luca) Verheul - S3704041
// Wed 13 Dec 2023

use crate::parser::bench_parse;
use crate::tokenizer::bench_tokenize;
use crate::type_checker::bench_type_check;

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

    let judgement = bench_parse(&tokens);

    // <Test TypeCheck>
    //
    // <warmup>
    println!("Warming up...");
    for _ in 0..1000000 {
        bench_type_check(&judgement);
    }
    println!("Done warming up.");
    // <test>
    let now3 = std::time::Instant::now();
    for _ in 0..times {
        bench_type_check(&judgement);
    }
    let elapsed3 = now3.elapsed();

    // <Test combined>
    //
    // <warmup>
    println!("Warming up...");
    for _ in 0..1000000 {
        bench_tokenize(&expression);
        bench_type_check(&bench_parse(&tokens));
    }
    println!("Done warming up.");
    // <test>
    let now4 = std::time::Instant::now();
    for _ in 0..times {
        bench_tokenize(&expression);
        bench_type_check(&bench_parse(&tokens));
    }
    let elapsed4 = now4.elapsed();
    // </Test combined>

    // print results
    println!(
        "Tokenizing {} {} times took {:?}",
        expression, times, elapsed1
    );
    println!("Parsing {} {} times took {:?}", expression, times, elapsed2);
    println!(
        "Typechecking {} {} times took {:?}",
        expression, times, elapsed3
    );
    println!(
        "Tokenizing, parsing and typechecking (combined) {} {} times took {:?}",
        expression, times, elapsed4
    );
}

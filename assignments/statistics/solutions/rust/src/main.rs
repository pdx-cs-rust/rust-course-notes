// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::process::exit;

///! Compute a statistic on numbers presented one-per-line on
///! standard input.

/// Report proper usage and exit.
fn usage() -> ! {
    eprintln!("stats: usage: stats [--mean|--stddev|--median|--l2]");
    exit(1);
}

/// Do the computation.
fn main() {
    // Process the argument.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        usage();
    }
    let target = &args[1];
    let argdescs: &[(&str, stats::StatFn)] = &[
        ("--mean", stats::mean),
        ("--stddev", stats::stddev),
        ("--median", stats::median),
        ("--l2", stats::l2),
    ];
    let stat = argdescs
        .iter()
        .find(|(a, _)| a == target)
        .unwrap_or_else(|| usage())
        .1;

    // Read the input.
    use std::io::BufRead;
    let nums: Vec<f64> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| {
            let s = s.unwrap_or_else(|e| {
                eprintln!("error reading input: {}", e);
                exit(-1);
            });
            s.parse::<f64>().unwrap_or_else(|e| {
                eprintln!("error parsing number {}: {}", s, e);
                exit(-1);
            })
        })
        .collect();

    // Run the stat, show the result if any.
    if let Some(result) = stat(&nums) {
        println!("{}", result);
    }
}

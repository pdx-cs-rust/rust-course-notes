//! CA runner — Bart Massey 2025
//!
//! Generalized solution to <https://github.com/pdx-cs-rust/hw-rule110>.

use clap::Parser;

use ca_rule::*;

/// Program arguments.
#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, default_value="*.*..*..")]
    /// Starting position string. Maximum 64 characters.
    start: String,
    #[arg(short, long, default_value="10")]
    /// Number of rows to print. Must be positive.
    nrows: usize,
    #[arg(short, long)]
    /// Print just last row.
    last: bool,
}

/// Make the rows of required output.
fn main() {
    let args = Args::parse();
    if args.nrows == 0 {
        return;
    }
    let mut cur = match Row::make_start(&args.start) {
        Ok(cur) => cur,
        Err(e) => {
            eprintln!("ca-rule: {}", e);
            std::process::exit(1);
        }
    };
    for _ in 0..args.nrows - 1 {
        if !args.last {
            println!("{}", cur.row_string());
        }
        cur = cur.next_row();
    }
    println!("{}", cur.row_string());
}

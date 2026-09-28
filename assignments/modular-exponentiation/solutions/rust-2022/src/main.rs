//! Command-line modular exponentiation tool.
//!
//! Bart Massey 2022

#[cfg(test)]
mod tests;

/// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    #[cfg(not(test))]
    std::process::exit(1);
    #[cfg(test)]
    panic!("error");
}

/// Parse the given string as a `u64`.
fn parsenum(s: &str) -> u64 {
    s.parse().unwrap_or_else(|_| error())
}

/// Efficiently compute `x**y mod m`.
/// `O(lg y)` runtime.
///
/// # Panics
/// Will panic if `m` is 0.
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    assert!(m > 0);
    if m == 1 {
        return 0;
    }
    let mut x = u128::from(x);
    let mut y = u128::from(y);
    let m = u128::from(m);
    let mut z = 1;
    while y > 0 {
        if y % 2 == 1 {
            z = (z * x) % m;
        }
        x = (x * x) % m;
        y /= 2;
    }
    u64::try_from(z).expect("internal error: too-large result")
}


fn run(args: Vec<String>) -> u64 {
    if args.len() != 4 {
        error();
    }
    let x = parsenum(&args[1]);
    let y = parsenum(&args[2]);
    let m = parsenum(&args[3]);
    if m == 0 {
        error();
    }
    modexp(x, y, m)
}

#[test]
fn test_run() {
    assert_eq!(1, run(vec![
        "".to_string(),
        "2".to_string(),
        "8".to_string(),
        "255".to_string(),
    ]));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{}", run(args));
}

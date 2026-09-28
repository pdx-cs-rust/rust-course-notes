//! Command-line modular exponentiation tool.

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
    assert!(x > 0 || y > 0);
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
    u64::try_from(z).unwrap()
}

#[test]
fn test_modexp() {
    // Largest prime less than 2**64.
    // https://primes.utm.edu/lists/2small/0bit.html
    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
    // https://practice.geeksforgeeks.org/problems/
    //    modular-exponentiation-for-large-numbers/0
    assert_eq!(4, modexp(10, 9, 6));
    assert_eq!(34, modexp(450, 768, 517));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        error();
    }
    let x = parsenum(&args[1]);
    let y = parsenum(&args[2]);
    let m = parsenum(&args[3]);
    if m == 0 {
        error();
    }
    println!("{}", modexp(x, y, m));
}

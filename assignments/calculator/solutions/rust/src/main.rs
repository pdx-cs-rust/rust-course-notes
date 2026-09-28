// Copyright © 2018 Jim Blandy, Jason Orendorff, Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


// Calculate some stuff based on command-line input.

use std::env;
use std::str::FromStr;

// Compute the GCD of two numbers.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t
        }
        m %= n
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    let n1 = 2 * 3 * 5 * 11 * 17;
    let n2 = 3 * 7 * 11 * 13 * 19;
    let d = 3 * 11;
    assert_eq!(gcd(n1, n2), d)
}

// Compute the LCM of two numbers.
fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(3, 2), 6);
    let n1 = 2 * 3 * 11 * 17;
    let n2 = 3 * 11 * 13 * 19;
    let d = 2 * 3 * 11 * 13 * 17 * 19;
    assert_eq!(lcm(n1, n2), d)
}

// Keep a running GCD of inputs and print as needed.
fn main() {
    let mut args = env::args();
    let _ = args.next();
    let prog = args.next().unwrap();
    let f = match &*prog {
        "gcd" => gcd,
        "lcm" => lcm,
        "sum" => |n, m| n + m,
        "product" => |n, m| n * m,
        _ => panic!("unknown function"),
    };
    let mut a = None;
    for arg in args {
        let arg = u64::from_str(&arg).expect("bad argument");
        a = match a {
            None => Some(arg),
            Some(a) => Some(f(a, arg)),
        }
    }
    if let Some(a) = a {
        println!("{}", a)
    }
}

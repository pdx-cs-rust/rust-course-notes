use std::convert::TryInto;
use toy_rsa_lib::*;

fn lambda(p: u32, q: u32) -> u64 {
    lcm((p - 1) as u64, (q - 1) as u64)
}

/// RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
///
/// # Panics:
/// Panics if key is 0.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg as u64, EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
///
/// # Panics:
/// Panics if either key component is 0. May panic if
/// `msg` is not valid ciphertext.
pub fn decrypt((p, q): (u32, u32), msg: u64) -> u32 {
    let d = modinverse(EXP, lambda(p, q));
    modexp(msg, d, p as u64 * q as u64)
        .try_into()
        .expect("invalid ciphertext")
}

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent
/// `EXP`. Warning: this routine has unbounded runtime; it
/// works by generate-and-test, generating pairs of primes
/// `p` `q` and testing that they satisfy `λ(pq) >= EXP` and
/// that `λ(pq)` has no common factors with `EXP`.
/// The prime generator is deterministic and predictable;
/// these keys are suitable only for teaching examples.
pub fn genkey() -> (u32, u32) {
    loop {
        let p = rsa_prime();
        let q = rsa_prime();
        let l = lambda(p, q);
        if p != q && EXP < l && gcd(EXP, l) == 1 {
            return (p, q);
        }
    }
}

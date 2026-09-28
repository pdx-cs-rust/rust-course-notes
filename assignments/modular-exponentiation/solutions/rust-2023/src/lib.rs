use std::sync::atomic::{AtomicU64, Ordering};

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b))
        .checked_mul(b)
        .expect("least common multiple exceeds u64")
}

fn modmul(a: u64, b: u64, modulus: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(modulus)) as u64
}

pub fn modexp(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    assert_ne!(modulus, 0, "modulus must be nonzero");
    base %= modulus;
    let mut result = 1 % modulus;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = modmul(result, base, modulus);
        }
        base = modmul(base, base, modulus);
        exponent >>= 1;
    }
    result
}

pub fn modinverse(value: u64, modulus: u64) -> u64 {
    assert_ne!(modulus, 0, "modulus must be nonzero");
    let (mut remainder, mut next_remainder) = (modulus, value % modulus);
    let (mut coefficient, mut next_coefficient) = (0, 1 % modulus);
    while next_remainder != 0 {
        let quotient = remainder / next_remainder;
        (remainder, next_remainder) = (next_remainder, remainder % next_remainder);
        let product = modmul(quotient, next_coefficient, modulus);
        let difference = (u128::from(coefficient) + u128::from(modulus) - u128::from(product))
            % u128::from(modulus);
        (coefficient, next_coefficient) = (next_coefficient, difference as u64);
    }
    assert_eq!(remainder, 1, "modular inverse does not exist");
    coefficient
}

fn next_random() -> u64 {
    static STATE: AtomicU64 = AtomicU64::new(0x243f_6a88_85a3_08d3);
    let mut value = STATE
        .fetch_add(0x9e37_79b9_7f4a_7c15, Ordering::Relaxed)
        .wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn is_prime(value: u32) -> bool {
    if value < 2 || value.is_multiple_of(2) {
        return value == 2;
    }
    let mut divisor = 3;
    while divisor <= value / divisor {
        if value.is_multiple_of(divisor) {
            return false;
        }
        divisor += 2;
    }
    true
}

/// Generates a prime in `2^31..2^32` for this teaching example.
///
/// Candidates come from SplitMix64 with a fixed seed and an atomic
/// process-local counter. The sequence is deterministic when calls are
/// made in the same order; concurrent calls share it safely. Trial
/// division verifies primality exactly. This generator is predictable
/// and must never be used to generate real cryptographic keys.
pub fn rsa_prime() -> u32 {
    loop {
        let candidate = next_random() as u32 | (1 << 31) | 1;
        if is_prime(candidate) {
            return candidate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_factors_and_multiples() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(u64::MAX, 0), u64::MAX);
        assert_eq!(lcm(0, 0), 0);
        assert_eq!(lcm(0, u64::MAX), 0);
        assert_eq!(lcm(54, 24), 216);
        assert_eq!(lcm(u64::MAX, u64::MAX), u64::MAX);
    }

    #[test]
    #[should_panic(expected = "least common multiple exceeds u64")]
    fn rejects_unrepresentable_multiple() {
        lcm(u64::MAX, 2);
    }

    #[test]
    fn exponentiation_handles_full_width_values() {
        assert_eq!(modexp(4, 13, 497), 445);
        assert_eq!(modexp(u64::MAX - 1, 2, u64::MAX), 1);
        assert_eq!(modexp(u64::MAX - 1, u64::MAX, u64::MAX), u64::MAX - 1);
        assert_eq!(modexp(0, 0, 7), 1);
        assert_eq!(modexp(u64::MAX, u64::MAX, 1), 0);
    }

    #[test]
    #[should_panic(expected = "modulus must be nonzero")]
    fn rejects_zero_exponentiation_modulus() {
        modexp(1, 2, 0);
    }

    #[test]
    fn inverse_handles_full_width_values() {
        assert_eq!(modinverse(17, 3120), 2753);
        assert_eq!(modinverse(2, u64::MAX), 1 << 63);
        assert_eq!(modinverse(u64::MAX - 1, u64::MAX), u64::MAX - 1);
        assert_eq!(modinverse(u64::MAX, 1), 0);
    }

    #[test]
    fn inverses_match_small_exhaustive_cases() {
        for modulus in 2..100 {
            for value in 1..modulus {
                if gcd(value, modulus) == 1 {
                    assert_eq!((value * modinverse(value, modulus)) % modulus, 1);
                }
            }
        }
    }

    #[test]
    #[should_panic(expected = "modular inverse does not exist")]
    fn rejects_noninvertible_value() {
        modinverse(6, 9);
    }

    #[test]
    #[should_panic(expected = "modulus must be nonzero")]
    fn rejects_zero_inverse_modulus() {
        modinverse(1, 0);
    }

    #[test]
    fn primality_handles_boundaries_and_composites() {
        for value in [0, 1, 4, 9, 25, 561, 1105, 1729, u32::MAX] {
            assert!(!is_prime(value), "{value}");
        }
        for value in [2, 3, 5, 17, 65_537, 2_147_483_647, 4_294_967_291] {
            assert!(is_prime(value), "{value}");
        }
    }

    #[test]
    fn generated_primes_are_in_range() {
        for _ in 0..16 {
            let prime = rsa_prime();
            assert!(prime >= 1 << 31);
            assert!(is_prime(prime));
        }
    }
}

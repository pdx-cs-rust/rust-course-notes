pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut count = 0;
    while n > 1 {
        n = match n {
            n if n & 1 == 0 => n / 2,
            n => 3 * n + 1,
        };
        count += 1;
    }
    Some(count)
}

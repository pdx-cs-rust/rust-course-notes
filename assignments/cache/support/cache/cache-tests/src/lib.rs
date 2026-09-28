/*!
Provide tests of a cache using Fibonacci numbers notionally
computed "the slow way". Depth is adjusted to cache capacity
or so.
*/

use cache::Cache;

fn fib_cache(cache: &mut dyn Cache<usize, Item = u64>, n: usize) -> u64 {
    if let Some(&mut f) = cache.retrieve(&n) {
        return f;
    }
    let f = match n {
        0 => 1,
        1 => 1,
        n => fib_cache(cache, n - 1) + fib_cache(cache, n - 2),
    };
    cache.insert(n, f);
    f
}

fn fib_dp(n: usize) -> u64 {
    let mut f0 = 1;
    let mut f1 = 1;
    for _ in 0..n {
        let f = f0 + f1;
        f0 = f1;
        f1 = f;
    }
    f0
}

#[test]
fn test_fib_dp() {
    assert_eq!(1, fib_dp(0));
    assert_eq!(1, fib_dp(1));
    assert_eq!(2, fib_dp(2));
    assert_eq!(3, fib_dp(3));
    assert_eq!(5, fib_dp(4));
}

pub fn test_fib_cache(mut cache: Box<dyn Cache<usize, Item = u64>>) {
    let k = cache.capacity().unwrap_or(40) + 3;
    assert_eq!(fib_dp(k), fib_cache(&mut *cache, k));
    for i in 0..=k {
        if let Some(&mut f) = cache.retrieve(&i) {
            assert_eq!(fib_dp(i), f);
        }
    }
    assert!(cache.retrieve(&(k + 1)).is_none());
}

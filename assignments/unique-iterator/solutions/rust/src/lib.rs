//! Find the unique value from an iterator satisfying a
//! predicate. (See the documentation for `Unique` for a full
//! example.)

/// Trait for applying `unique()` to an `Iterator` (or other
/// object).
pub trait Unique<T> {
    fn unique<P>(&mut self, pred: P) -> Option<T>
    where
        P: FnMut(&T) -> bool;
}

// Documentation largely taken from `std::iter::Iterator::find()`.

/// Searches for a unique element of an iterator that satisfies a
/// predicate.
///
/// `unique()` takes a closure that returns `true` or
/// `false`. It applies this closure to each element of the
/// iterator, and if exactly one of them return `true`, then
/// `unique()` returns `Some(element)`. If they all return
/// `false`, or if more than one returns `true`, it returns
/// `None`.
///
/// `unique()` is short-circuiting; in other words, it will
/// stop processing as soon as the closure returns `true` a
/// second time.
///
/// Because `unique()` takes a reference, and many iterators iterate
/// over references, this leads to a possibly confusing
/// situation where the argument is a double reference. You can
/// see this effect in the example below, with `&&n`.
///
/// # Examples
///
/// ```
///    use unique::Unique;
///    let mut nums = vec![];
///    let even: fn(&&usize) -> bool = |&&n| n % 2 == 0;
///    assert_eq!(None, nums.iter().unique(even));
///    nums.push(1);
///    assert_eq!(None, nums.iter().unique(even));
///    nums.push(0);
///    assert_eq!(Some(&0), nums.iter().unique(even));
///    nums.push(3);
///    nums.push(2);
///    nums.push(5);
///    assert_eq!(None, nums.iter().unique(even));
/// ```
impl<T, I> Unique<T> for I
where
    I: Iterator<Item = T>,
{
    fn unique<P>(&mut self, mut pred: P) -> Option<T>
    where
        P: FnMut(&T) -> bool,
    {
        let mut result = None;
        for v in self {
            if pred(&v) {
                if result.is_some() {
                    return None;
                }
                result = Some(v);
            }
        }
        result
    }
}

// Ensure that `unique()` will work with non-address types.
#[test]
fn test_unique_into_iter() {
    let nums = vec![1, 2, 3];
    let even: fn(&usize) -> bool = |&n| n % 2 == 0;
    assert_eq!(Some(2), nums.into_iter().unique(even));
}

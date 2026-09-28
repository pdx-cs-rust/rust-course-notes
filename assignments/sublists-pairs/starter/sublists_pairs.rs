// Thanks to a Redditor for an interesting problem.
// (Details on request.)

/*!
Sometimes you need to deal with less-than-ideal input. In
this assignment, someone is going to pass you a list of
lists of values. The values in the sublists are treated as
"paired up" — the sublists should always be of even length,
otherwise it's an input error. Your job is to iterate over
this list of lists and produce pairs of values (in the order
encountered) from the sublists.

For example, given
```text
[[4, 2, 3, 1], [5, 6]]
```
your iterator should produce
```text
(4, 2) (3, 1) (5, 6)
```
Given
```text
[[1, 2, 3], [4]]
```
your iterator should produce
```text
(1, 2) Error
```
*/

use thiserror::Error;

/// Error cases for [`sublists_pairs`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SublistsPairsError {
    /// Sublist has an odd number of elements.
    #[error("incomplete sublist (odd number of elements)")]
    OddSublistError,
}

/**
Collect a vec of sublists, returning an iterator pairing
the elements of each sublist into two-tuples and returning
the resulting two-tuples in order.

# Examples

```
    # use sublists_pairs::*;
    let v: Result<Vec<(usize, usize)>, SublistsPairsError> =
        sublists_pairs(vec![vec![1, 2], vec![3, 4, 5, 6]]).collect();
    assert_eq!(v.unwrap(), [(1, 2), (3, 4), (5, 6)]);
    let v: Result<Vec<(usize, usize)>, SublistsPairsError> =
        sublists_pairs(vec![vec![1, 2, 3], vec![4, 5, 6, 7]]).collect();
    assert!(v == Err(SublistsPairsError::OddSublistError));
```
*/
pub fn sublists_pairs<T>(
    vals: Vec<Vec<T>>,
) -> impl Iterator<Item = Result<(T, T), SublistsPairsError>> {
    todo!()
}

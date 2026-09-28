//! # Assignment
//!
//! * Constrain the type of `K` sufficiently to make this
//!   code compile and run.
//!
//! * Implement `retrieve()` and `insert()`. Ideally
//!   these will each be one line involving the `HashMap`.
//!
//! * Add a test that shows that inserting an item *i*
//!   with key *k* correctly overwrites an older item *i'*
//!   at key *k*.
use cache::*;

use std::collections::HashMap;
use std::hash::Hash;

/// Cache that grows without bound: all elements that get
/// inserted are retained indefinitely. This is not a
/// realistic policy for large or long-running computations.
pub struct NonEvictingCache<K, I> {
    elems: HashMap<K, I>,
}

impl<K, I> NonEvictingCache<K, I> {
    /// Make a new non-evicting cache with the given *initial* capacity.
    pub fn new(capacity: usize) -> Self {
        let elems = HashMap::with_capacity(capacity);
        NonEvictingCache { elems }
    }

    /// Insert the given item into the cache at the given
    /// key, replacing whatever was at that key.
    pub fn insert(&mut self, key: K, item: I) {
        todo!()
    }

    /// Get a mutable reference to the item associated with
    /// the given key from the cache, if any.
    pub fn retrieve(&mut self, key: &K) -> Option<&mut I> {
        todo!()
    }

    /// Report the lack of a capacity limit for the cache.
    pub fn capacity(&self) -> Option<usize> {
        None
    }
}

#[test]
fn test_non_evicting() {
    let mut nev = NonEvictingCache::new(3);
    nev.insert("a", 0u8);
    nev.insert("b", 1);
    nev.insert("c", 2);
    nev.insert("d", 3);
    assert_eq!(Some(&mut 0), nev.retrieve(&"a"));
    assert_eq!(Some(&mut 1), nev.retrieve(&"b"));
    assert_eq!(Some(&mut 2), nev.retrieve(&"c"));
    assert_eq!(Some(&mut 3), nev.retrieve(&"d"));
    todo!()
}

impl<K, I> Cache<K> for NonEvictingCache<K, I> {
    type Item = I;

    fn insert(&mut self, key: K, item: Self::Item) {
        self.insert(key, item)
    }

    fn retrieve(&mut self, key: &K) -> Option<&mut Self::Item> {
        self.retrieve(key)
    }

    fn capacity(&self) -> Option<usize> {
        None
    }
}

#[test]
fn test_non_evicting_cache() {
    let cache = Box::new(NonEvictingCache::new(40));
    cache_tests::test_fib_cache(cache);
}

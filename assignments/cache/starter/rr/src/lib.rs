//! # Assignment
//!
//! * Finish the definition of `insert()`.
//!
//! * Finish the definition of `retrieve()`.
//!
//! * Comment `test_rr()`.
use cache::*;

use std::collections::HashMap;
use std::hash::Hash;

use fastrand::Rng;

/// A cache with random-replacement eviction.
///
/// General strategy is to keep items in a vector along with their keys;
/// a map is also kept from keys to indices into the vector. This allows
/// all values to be owned by the cache, at the expense of keys needing to be
/// `Clone`.
///
/// Retrieval is done by looking up the key in the map, looking
/// up the retrieved value in the vector, and then checking that
/// the cached key matches.
///

/// Insertion proceeds in two cases:
///
/// * If the cache is not yet full, the value and its key
///   are pushed onto the vector, and the hashmap is updated
///   to point to them.
///
/// * If the cache is full, a random element is selected for
///   replacement.  It is replaced with the key and value
///   being inserted, and the map entry is updated.
///
/// Random numbers are generated using a
/// deterministically-seeded PRNG stored with the
/// cache. This allows reproducible results for testing, at
/// the expense of being vulnerable to certain security
/// issues.
pub struct RrCache<K, I> {
    map: HashMap<K, usize>,
    elems: Vec<(K, I)>,
    capacity: usize,
    rng: Rng,
}

impl<K: Hash + Eq + Clone, I> RrCache<K, I> {
    /// Make a new random-replacement cache with the given
    /// capacity.
    pub fn new(capacity: usize) -> Self {
        let rng = Rng::with_seed(0x12345678);
        let map = HashMap::with_capacity(capacity);
        let elems = Vec::with_capacity(capacity);
        RrCache { map, elems, capacity, rng }
    }

    /// Insert an item into the cache, replacing a random
    /// item if needed to not exceed capacity.
    pub fn insert(&mut self, key: K, item: I) {
        let n = self.elems.len();
        let i = if n < self.capacity {
            self.elems.push((key.clone(), item));
            n
        } else {
            todo!()
        };
        self.map.insert(key, i);
    }

    /// Get the item at the given key (if any) from the cache.
    pub fn retrieve(&mut self, key: &K) -> Option<&mut I> {
        let &i = self.map.get(key)?;
        let (ref ekey, ref mut item) = self.elems[i];
        todo!()
    }

    /// Report the capacity of the cache.
    pub fn capacity(&self) -> Option<usize> {
        Some(self.capacity)
    }
}

#[test]
fn test_rr() {
    let mut rr = RrCache::new(3);
    let rng = Rng::with_seed(0x12345678);
    rr.insert("a", 0u8);
    rr.insert("b", 1);
    rr.insert("c", 2);
    assert_eq!(Some(&mut 0), rr.retrieve(&"a"));
    assert_eq!(Some(&mut 1), rr.retrieve(&"b"));
    assert_eq!(Some(&mut 2), rr.retrieve(&"c"));
    let i_d = rng.usize(0..3);
    let k_i_d = rr.elems[i_d].0;
    rr.insert("d", 3);
    assert_eq!(rr.elems[i_d], ("d", 3));
    assert_eq!(Some(&mut 3), rr.retrieve(&"d"));
    assert!(rr.retrieve(&"e").is_none());
    assert!(rr.retrieve(&k_i_d).is_none());
}

impl<K: Hash + Eq + Clone, I> Cache<K> for RrCache<K, I> {
    type Item = I;

    fn insert(&mut self, key: K, item: Self::Item) {
        self.insert(key, item)
    }

    fn retrieve(&mut self, key: &K) -> Option<&mut Self::Item> {
        self.retrieve(key)
    }

    fn capacity(&self) -> Option<usize> {
        Some(self.capacity)
    }
}

#[test]
fn test_rr_cache() {
    let cache = Box::new(RrCache::new(40));
    cache_tests::test_fib_cache(cache);
}

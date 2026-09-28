//! # Assignment
//!
//! * Add a comment at the indicated place in the
//!   `advance_time()` method describing why `checked_sub()`
//!   is necessary here.
//!
//! * Finish the definition of `insert()`.
use cache::*;

use std::hash::Hash;

use keyed_priority_queue::KeyedPriorityQueue;

/// A cache with Least-Recently-Used eviction.
///
/// General strategy is to keep cache entries in a priority
/// queue ordered by "time" of most recent use.  A "query
/// clock" increments each time a value is stored or
/// requested.
///
/// When a value is accessed, its position in the priority
/// queue is adjusted. When some value needs to be evicted,
/// the victim is pulled out of the priority queue.
pub struct LRU<K, I>
where
    K: Hash + Eq,
{
    capacity: usize,
    time: u64,
    store: Vec<I>,
    usage: KeyedPriorityQueue<K, (u64, usize)>,
}

impl<K, I> LRU<K, I>
where
    K: Hash + Eq,
{
    fn advance_time(&mut self) -> u64 {
        let t = self.time;
        // XXX Comment this.
        self.time = t.checked_sub(1).unwrap();
        t
    }

    /// Make a new LRU cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        let time = !0;
        let store = Vec::with_capacity(capacity);
        let usage = KeyedPriorityQueue::with_capacity(capacity);
        Self {
            capacity,
            time,
            store,
            usage,
        }
    }

    /// Insert the given item into the cache at the given
    /// key, evicting if necessary.
    pub fn insert(&mut self, key: K, item: I) {
        let t = self.advance_time();
        let n = self.store.len();
        let i = if n < self.capacity {
            // Find an index for the item in the store and
            // save it there. No need to evict, since
            // there's still room.
            todo!()
        } else {
            let (_, (t0, index)) = self.usage.pop().unwrap();
            assert!(t0 > t);
            self.store[index] = item;
            index
        };
        self.usage.push(key, (t, i));
    }

    /// Get a mutable reference to the item associated with
    /// the given key from the cache, if any.
    pub fn retrieve(&mut self, key: &K) -> Option<&mut I> {
        // XXX Perhaps slightly faster to adjust the
        // priority than to remove and reinsert the entry.
        let (key, (_, index)) = self.usage.remove_entry(key)?;
        let t = self.advance_time();
        self.usage.push(key, (t, index));
        Some(&mut self.store[index])
    }

    /// Report the set capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[test]
fn test_lru() {
    let mut lru = LRU::new(3);
    lru.insert("a", 0u8);
    lru.insert("b", 1);
    lru.insert("c", 2);
    lru.insert("d", 3);
    assert!(lru.retrieve(&"a").is_none());
    assert_eq!(Some(&mut 1), lru.retrieve(&"b"));
    lru.insert("e", 4);
    assert_eq!(Some(&mut 1), lru.retrieve(&"b"));
    assert!(lru.retrieve(&"c").is_none());
}

impl<K, I> Cache<K> for LRU<K, I>
where
    K: Hash + Eq,
{
    type Item = I;

    fn insert(&mut self, key: K, item: Self::Item) {
        self.insert(key, item)
    }

    fn retrieve(&mut self, key: &K) -> Option<&mut Self::Item> {
        self.retrieve(key)
    }

    fn capacity(&self) -> Option<usize> {
        Some(self.capacity())
    }
}

#[test]
fn test_lru_cache() {
    let cache = Box::new(LRU::new(40));
    cache_tests::test_fib_cache(cache);
}

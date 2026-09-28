# cache: Cache Trait Library Crate
Bart Massey 2022 (version 0.1.0)

This crate provides a `Cache` trait that can be used to
provide a common interface for cache implementations.  The
associated `cache-tests` crate provides some generic cache
tests.

## Background

A *cache* is a common structure in computing. It manifests
as a memory store that sits "in front of" some collection of
values that are difficult or expensive to produce. When a
value is requested, it may be present in the cache, in which
case it can be quickly returned. Otherwise the produced
value may be stored in the cache for later reference.

Typically a cache has too little storage to contain all the
values that might be produced during a computation. Thus an
*eviction policy* is needed, specifying which of the
competing values should be retained.

The `Cache` trait here abstracts over caches with different
capacities and eviction policies, providing a simple
interface.

## Examples

```rust
use cache::Cache;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default)]
struct HashMapCache<K, I>(HashMap<K, I>);

impl<K: Hash + Eq, I> HashMapCache<K, I> {
    fn insert(&mut self, key: K, item: I) {
        self.0.insert(key, item);
    }
    fn retrieve(&mut self, key: &K) -> Option<&mut I> {
        self.0.get_mut(&key)
    }
}

impl<K: Hash + Eq, I> Cache<K> for HashMapCache<K, I> {
    type Item = I;

    fn insert(&mut self, key: K, item: I) {
        self.insert(key, item);
    }
    fn retrieve(&mut self, key: &K) -> Option<&mut I> {
        self.retrieve(key)
    }
}

cache_tests::test_fib_cache(Box::new(HashMapCache::default()));
```

Full crate
[rustdoc](https://pdx-cs-rust.github.io/cache/cache/index.html)
is available.

This crate is made available under the "MIT
license". Please see the file `LICENSE` in this distribution
for license terms.

Thanks to the `cargo-readme` crate for generation of this
`README`.

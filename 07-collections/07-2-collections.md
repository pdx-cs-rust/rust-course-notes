## Vec, Array: Review

* Arrays, Vecs and slices can only contain sized objects
  of a single type.  They are accessed by computing a
  fixed offset using the usual index calculation

* An "array" has a type that includes its size.
  It is an object, not a reference: stored on the stack
  sometimes, so watch out

* A Vec contains a pointer to heap-allocated storage, its
  length and its capacity. Vecs can be grown or shrunk

## Slice: Review

* A slice reference is a fat pointer with length, but points
  to an arbitrary chunk of memory (also called "slice"
  **arrgh**). It cannot be resized, and borrows the memory
  it points to

* References to Vecs and arrays can normally be used as
  slice references. An explicit cast may be needed in some
  contexts

* <https://github.com/pdx-cs-rust/reorder>

## Vec, Array, Slice: Methods

* Vec makes a good stack. Use the `push`, `pop` and
  `is_empty` methods

* The Vec `retain_mut` and `dedup` methods are pretty handy. The
  book describes the non-sorted dedup trick

* The `split` and `join` methods are used a lot in practice.
  `chunks` and `windows` are really handy in a lot of kinds
  of analysis, esp `chunks_exact`

* Sorting and searching have straightforward methods. Having
  `binary_search` and `partition_point` built-in is a
  fantastic idea, as this is a common source of bugs in
  other languages

* Lexicographic comparison of these is supported when `Ord`

## VecDeque

* `VecDeque` is a growable circular queue stored in a `Vec`

* Normally insert with `push_back()`, remove with
  `pop_front()`, but the queue is double-ended

  <https://github.com/PoHuit/plan-b>

## LinkedList

* `LinkedList` is a doubly-linked list

* Don't use it unless you have to: its memory performance is
  terrible and it's pretty error-prone

* Its API is intentionally sparse; the `Cursor` API is still
  available only on nightly

  <https://rust-unofficial.github.io/too-many-lists/>

## BinaryHeap

* A heap intended for use as a priority queue

* A max-heap, which is annoying. I've used the
  `min-max-heap` crate successfully

* Not a keyed heap: no way to mess with an element in the
  middle. Compare with the `keyed_priority_queue` crate

## HashMap, HashSet, BTreeMap, BTreeSet

* Sets are just maps with `()` for content. Since `()` is a
  zero-sized type, this works fine

* `Hash` variants are open-addressed hash tables. Lots of
  extra storage (but arguably not enough). Cost of hashing,
  bad memory locality. Still, "constant-time" access

* `BTree` variants are, well, B-trees. Storage-efficient,
  better memory locality, but require a tree traversal

* `BTree` variants can be keys because they are
  `Ord`. `Hash` variants cannot because they are not `Hash`.

## Map / Set Stuff

* Ownership and mutability matter here. A map owns its keys
  and values: there is no way to mutate the key in-place

* The `Entry` interface avoids some extra lookups by getting
  a key-value pair that can be modified or inserted. It's an
  `enum` that depends on whether the entry currently is in
  the map. Normally, you will use the many methods provided
  for manipulating entries: this is classic combinator-chain
  stuff

  [`examples/histogram.rs`][histogram]

* Sets include the usual set operators with infix
  equivalents. `is_subset()` is not defined as infix `<=`

## Hashing

* It's a complicated mess. The default hasher is a
  compromise between security and performance

* You can specify your own hasher, but this is rarely
  desirable

* Using a built-in hasher is non-trivial

## Third-Party Crates

* Some stuff on <https://crates.io> is actually a pretty
  accepted part of the Rust ecosystem
  
* This is problematic: hard to tell the "really standard"
  stuff from the good stuff from the bad stuff
  
* Worth looking at a couple of examples

* The Mandelbrot demo from early in the book uses
  `crossbeam` and `complex`.

## fastrand

* A simple, lightweight crate for random number generation

* <https://docs.rs/fastrand/2.5.0/fastrand>

* Uses a fast pseudo-random generator that is *not*
  cryptographically secure

* Good for exercises, games, randomized testing, and
  simulations; not for keys, tokens, or security work

* Provides simple functions named for the desired result type

          fn main() {
              for _ in 0..10 {
                  println!("{}", fastrand::i32(1..=6));
              }
          }

* An explicit generator can be seeded for reproducible results

          let mut rng = fastrand::Rng::with_seed(7);
          println!("{}", rng.i32(1..=6));

## What About rand?

* The [`rand` crate](https://docs.rs/rand/latest/rand/) is
  still important in the Rust ecosystem

* It provides a broad framework of generators, distributions,
  traits, and cryptographically secure options

* It was long the obvious gold-standard default. As its scope
  and dependency weight have increased and its interface has
  churned, it has become less attractive for simple jobs

* Use `rand` when its larger ecosystem or more sophisticated
  facilities are useful; use `fastrand` for simple,
  non-cryptographic randomness


## serde

* Several crates for serializing and deserializing data:
  converting between Rust data structures and some kind
  of input/output format
  
* Huge variety of serialization formats supported

* <https://github.com/PoHuit/plan-b>

## So Many Crates

* The working space of `crates.io` (+ Github) for practical
  Rust is huge

* <https://github.com/rust-unofficial/awesome-rust> is… ok

* Mostly no great way to find "the greats"

[histogram]: https://github.com/pdx-cs-rust/examples/blob/main/histogram.rs

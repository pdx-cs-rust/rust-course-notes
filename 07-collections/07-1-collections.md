## Vec, Array: Review

* Arrays, Vecs and slices can only contain sized objects
  of a single type.  They are accessed by computing a
  fixed offset using the usual index calculation

* An "array" has a type that includes its size.
  It is an object, not a reference: stored on the stack
  sometimes, so watch out

* A Vec is a fat pointer to a chunk of heap; the Vec
  includes the length of the pointed-to chunk. Vecs
  can be grown or shrunk

## Slice: Review

* A slice is also a fat pointer with length, but points to
  an arbitrary chunk of memory (also called "slice"
  **arrgh**). It can thus not be resized, and has a borrow
  of the memory it points to

* You can make a slice from a Vec or array by `&`, because
  they implement `Deref`. (Also `&mut`, because `DerefMut`.)
  Note that a cast may be necessary in some contexts,
  because a reference to an array or slice is also a
  legitimate thing

* <http://github.com/pdx-cs-rust/reorder>

## Vec, Array, Slice: Methods

* Vec makes a good stack. Use the `push`, `pop` and
  `is_empty` methods

* The Vec `retain_mut` and `dedup` methods are pretty handy. The
  book describes the non-sorted dedup trick

* The `split` and `join` methods are used a lot in practice.
  `chunks` and `windows` are really handy in a lot of kinds
  of analysis, esp `chunks_exact`

* Sorting and searching have straightforward methods. Having
  `binary_search` built-in is a fantastic idea, as this is a
  common source of bugs in other languages

* Lexicographic comparison of these is supported when `Ord`

## VecDeque

* `VecDeque` is a circular queue stored in a `Vec`

* Because it uses stop and start pointers, there will always
  be at least one dead cell

* Normally insert with `push_back()`, remove with
  `pop_front()`, but the queue is double-ended

  <http://github.com/PoHuit/plan-b>

## LinkedList

* `LinkedList` is a doubly-linked list

* Don't use it unless you have to: its memory performance is
  terrible and it's pretty error-prone

* As of Rust 1.58 its API is still quite sparse, although
  the `Cursor` in nightly will help

  <http://cglab.ca/~abeinges/blah/too-many-lists/book/>

## BinaryHeap

* A heap intended for use as a priority queue

* A max-heap, which is annoying. I've used the
  `min-max-heap` crate successfully

* Not a keyed heap: no way to mess with an element in the
  middle. Compare with the `keyed_priority_queue` crate

## HashMap, HashSet, BTreeMap, BTreeSet

* Sets are just maps with `()` for content. Since `()` is a
  zero-sized type, this works fine

* `Hash` variants are open hash tables. Lots of extra
  storage (but arguably not enough). Cost of hashing, bad
  memory locality. Still, "constant-time" access

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

  `examples/histogram.rs`

* Sets include the usual set operators with infix
  equivalents. `is_subset()` is not defined as infix `<=`

## Hashing

* It's a complicated mess. The default hasher is a
  compromise between security and performance

* You can specify your own hasher, but this is rarely
  desirable

* Using a built-in hasher is non-trivial

## Third-Party Crates

* Some stuff on <http://crates.io> is actually a pretty
  accepted part of the Rust ecosystem
  
* This is problematic: hard to tell the "really standard"
  stuff from the good stuff from the bad stuff
  
* Worth looking at a couple of examples

* The Mandelbrot demo from early in the book uses
  `crossbeam` and `complex`.

## rand

* Crate for random number generation and use

* <https://docs.rs/rand/0.8.3/rand>

* Produces both pseudo-random and (if possible)
  hardware-random numbers
  
* Key entry point is `rand::thread_rng()`, which gives a
  per-thread OK-secure PRNG seeded from hardware
  
* You want the `gen_range(*lower*..=*upper*)` method
  normally, for uniform random numbers in a range.
  This requires the `rand::Rng` trait
  
* Here's an example:

          use rand::Rng;

          fn main() {
              let mut rng = rand::thread_rng();
              for _ in 0..10 {
                  println!("{}", rng.gen_range(1..=6));
              }
          }


## serde

* Several crates for serializing and deserializing data:
  converting between Rust data structures and some kind
  of input/output format
  
* Huge variety of serialization formats supported

* <http://github.com/PoHuit/plan-b>

## So Many Crates

* The working space of `crates.io` (+ Github) for practical
  Rust is huge

* <https://github.com/rust-unofficial/awesome-rust> is… ok

* Mostly no great way to find "the greats"

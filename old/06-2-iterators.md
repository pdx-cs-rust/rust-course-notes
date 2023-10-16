## Iterators

* `Iterator` is a trait, with an `Item` associated type

* Produces a sequence of `Option<Item>` values on calls to
  its `next()` method

* Ubiquitous in Rust code

## Iterator Adapters

* Implementation of `Iterator()` that operates by consuming
  the output of some passed iterator; essentially iterators
  over iterators

* Ubiquitous in Rust code

* Book chapter has a bestiary of Standard Library stuff,
  worth studying in detail

## FromIterator

* Trait used by `collect()` to make an iterator into a collection

* Can implement for your collection type to make it collectible

## IntoIterator

* Trait for getting a "canonical" iterator for a type

* Makes the type work with `for` loops

          trait IntoIterator where Self::IntoIter::Item == Self::Item {
              type Item;
              type IntoIter: Iterator;
              fn into_iter(self) -> Self::IntoIter;
          }

* Every `Iterator` has a default implementation of
  `IntoIterator`, so that you can `for` loop over an
  explicit iterator

## Implementing an Iterator

* Let's build a shuffling iterator over a slice: returns
  references to slice elements in random order

* `examples/shuflr.rs`

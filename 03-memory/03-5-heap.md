## The Heap

* In Rust, we put values on the heap with the `Box<T>` type.

* We can also use the `RefCell` (and `Cell`) type.

## RefCell

* Allows you to get the compiler to not do compile-time
  borrow checking, but instead do it at runtime.

## Interior Mutability and Reference Counting

* Rust borrow checker is quite conservative in its static
  rules for ownership

  * References must not outlive referents

  * Reader-writer rules

## Static Mutability Model

* Compile-time reader/writer model

  * One mut ref or many shared refs

  * Owner participates in model: cannot read while mut ref
    is shared, cannot write while ref is shared

* This makes some data structures hard to build

  * Even a simple counter can't really be shared by two data
    structures under these rules: if either parent is able
    to mutate it, the other can't even read it

## Idea 1: Interior Mutability

* Maybe we could make it so we sometimes do writes into an
  object without having mutable access to it?

* This sounds dangerous: the whole point of the borrow
  checker is to keep two things from read-modify-writing the
  same location at the same time

* Compromise: Check *at run time* that the borrow rules are
  followed. Then code can read-modify-write, drop the
  reference, and someone else can start in

## Cell/ RefCell

* Can implement this with a wrapper data structure `Cell` /
  `RefCell` that tracks borrowing

    * Call `.get()` to get a `Ref` "guard", which can
      dereference as an immutable ref

    * Call `.get_mut()` to get a `RefMut` "guard", which can
      dereference as an mutable ref

    * If you try to call `.get_mut()` when there's a guard
      out already that hasn't been dropped, or if you try to
      call `.get()` when there's a `RefMut` guard out, *your
      program will panic!*

* Hazard: If the borrow rules are violated at run time, it's a
  panic. The compiler can't help anymore

## Shedding Dead Cells

* Now we have a way to keep a mutable shared value around
  indefinitely: stick it in a `RefCell` with global scope

* In general, some value inside the `RefCell` must live as
  long as the `RefCell` itself: if you manually drop it,
  you must replace it to keep the `RefCell` initialized

* `Option` can kind of fix this, but you still have to
  manually replace the contained value to drop it

* This sounds like a great way to leak stuff

## Idea 2: GC

* Maybe we could track actual lifetime of objects at runtime
  instead of having the compiler over-approximate it?

* This sounds hard: this plan normally requires a garbage
  collector, and those are expensive and need language
  support

## Idea 3: Reference Counting

* Compromise: Use *reference counting*; keep track of a
  count of the number of references to an object and drop it
  when the count goes to 0

* Can implement this with a wrapper data structure that does
  the reference counting

* Hazard: it is possible to create reference cycles, which
  keep the object alive until the last reference is dropped

## Rc

* Rust provides `rc::Rc` as a reference-count wrapper

* Cloning the `Rc` does *not* clone its contents: it just
  hands out another pointer to itself and increments
  the refcount

* Dropping an `Rc` will decrement its refcount, and drop
  itself and its contained value iff the refcount becomes
  zero

## Example

* http://github.com/pdx-cs-rust/rc-ledger

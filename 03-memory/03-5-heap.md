## The Heap

* The "heap" is the region of memory managed at runtime. In
  C this is where `malloc()` and `free()` happen. The same
  is true in Rust.

* `Box<T>` is the simplest way to put a value on the heap

  ```rust
  let answer = Box::new(42);
  println!("{}", answer);
  ```

  This implicitly calls something like `malloc()` and initializes
  the resulting storage; in Rust, these are inseparable by design,
  unlike in C, where "accidents" can happen

* The `Box` owns both the allocation and its contents. Under
  normal execution, when the `Box` is dropped, both go away.
  Ownership prevents double-free and use-after-free. Rust
  does not prevent memory leaks, but it does make accidental
  leaks less likely

* Types such as `Vec`, `String`, and `Rc` also manage heap
  allocations internally

* `Cell` and `RefCell` provide interior mutability; they do
  not themselves put their contents on the heap

## One Writer Or Many Readers

* The usual borrow rule is one mutable reference or any
  number of shared references

* This can make shared mutable data awkward. If two parts of
  a program share a counter, who gets mutable access to it?

* Interior mutability provides controlled ways to mutate a
  value through a shared reference

## Cell

* `Cell<T>` works well for small values that can be copied or
  replaced as a whole

  ```rust
  use std::cell::Cell;

  let score = Cell::new(0);
  let shared = &score;

  shared.set(shared.get() + 1);
  println!("{}", score.get());
  ```

* `Cell` never hands out references to its contents

  * `.get()` copies out the value when `T` is `Copy`

  * `.set()`, `.replace()`, and `.take()` replace the value

## RefCell

* `RefCell<T>` is useful when code needs temporary references
  to the value inside

  ```rust
  use std::cell::RefCell;

  let names = RefCell::new(vec![String::from("Ada")]);

  {
      let mut names = names.borrow_mut();
      names.push(String::from("Grace"));
  }

  println!("{:?}", names.borrow());
  ```

* The inner block makes it explicit that the mutable guard is
  dropped before the later shared borrow

* `.borrow()` returns a shared guard; `.borrow_mut()` returns
  an exclusive guard

* The guards enforce the usual borrow rules at runtime. An
  incompatible borrow while a guard is live causes a panic

* `Cell` and `RefCell` do not provide thread-safe sharing. We
  will see thread-safe alternatives later

## Shedding Dead Cells

* A `Cell` or `RefCell` must always contain an initialized
  value, even when moving the old value out

* `RefCell<Option<T>>` lets you take the contained value,
  leaving `None` in its place

  ```rust
  use std::cell::RefCell;

  let message =
      RefCell::new(Some(String::from("hello")));
  let old_message = message.borrow_mut().take();

  println!("{:?}", old_message);
  println!("{:?}", message.borrow());
  ```

## Shared Ownership With Rc

* Many languages use garbage collection for shared ownership.
  Rust also offers reference counting for this job

* `Rc<T>` owns a heap allocation. Cloning the `Rc` makes
  another owner of that same allocation

  ```rust
  use std::rc::Rc;

  let first = Rc::new(String::from("hello"));
  let second = Rc::clone(&first);

  println!("{} {}", first, second);
  ```

* Dropping an `Rc` decrements the count. The contents are
  dropped after the last owner goes away

* Reference cycles can leak because their counts never
  reach zero, even after the last external reference goes away.

## Rc + RefCell

* These mechanisms can be composed when several owners need
  to mutate the same value

  ```rust
  use std::cell::RefCell;
  use std::rc::Rc;

  let log = Rc::new(RefCell::new(Vec::new()));
  let parser = Rc::clone(&log);
  let reporter = Rc::clone(&log);

  parser.borrow_mut().push("parsed input");
  reporter.borrow_mut().push("reported result");

  println!("{:?}", log.borrow());
  ```

* All three `Rc` values own the same allocation; the `RefCell`
  checks mutable access to its contents

## Larger Example

* <https://github.com/pdx-cs-rust/rc-ledger>

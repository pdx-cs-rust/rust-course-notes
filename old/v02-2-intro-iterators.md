## Iterators and Closures

* Basics are here

* The full story is much complicated; we will cover it
  extensively later in the course

## Iterators

* Value that lazily produces values via a `.next()`
  method. The `.next()` method returns an `Option`: `Some`
  while the iterator is still running, `None` when it is
  done

* Obvious example is via the `..` operator:

         let mut i = 0..3;
         while let Some(n) = i.next() {
             println!("{}", n);
         }

* Rust `..` excludes the last value, like C. Use `..=` to
  include it.

## For / While

* The previous is exactly the same as

         for n in 0..3 {
             println!("{}", n);
         }

## Iterator Methods

* A whole pile of methods exist for working with iterators
  ("functional-style" programming), for example

        println!("{}", (1..5).sum());

  Many methods make a new iterator from an old, for
  example

        for i in (0..5).rev() {
            println!("{}", i);
        }

## Closures

* Anonymous functions ("lambdas") that can reference local
  variables

        // Squaring function.
        |n| n * n

* Unlike `fn`, type inference is attempted on closure
  arguments and result

* Type is almost "first-class": values can be stored in
  variables, passed around, etc

* Normally use with methods that want to do a computation
  as part of their action

        let g2 = (1..5).map(|n| n * n).sum();


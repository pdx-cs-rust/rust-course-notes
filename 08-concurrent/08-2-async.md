* Book chapter 20
* <https://rust-lang.github.io/async-book/>
* <https://tokio.rs/tokio/tutorial>
* <https://book.async.rs/>

## The Problem With Processes

* Processes are a great concurrency / parallelism mechanism

    * Fantastic isolation of control flow and data
    * Simple semantics
    * OS deals with them well

* But processes are inefficient

    * Inter-process communication is inefficient and clumsy
    * Memory usage tends toward inefficiency

* So threads

## The Problem With Threads

* Threads are a decent concurrency / parallelism mechanism

    * Control flow isolation
    * Manageable semantics
    * OS deals with them OK
    * Way more efficient than processes

* But threads are still inefficient

    * Startup costs
    * Context switch costs
    * Stacks-per-thread
    * Inter-thread locking

## Task Pools

* Idea: Small number of threads executing large number of
  short tasks

    * Minimize context switch costs
    * Still stack per thread, but not many of those
    * Synchronization can partly be done with tasks rather than locks

* Mechanism: Task Pool — a collection of ready-to-run tasks

    * Worker fetches task from pool and executes it until
        * Task finishes
        * Task is going to block — put it aside after
          arranging for it to be put back in the pool
          when it can proceed (not easy)

## Futures

* Old idea: a closure that will "continue running" a
  computation when called

* Futures eventually deliver a value that is the result of
  computation

* Compare with delay/force, promises, generators,
  continuations, etc

* Represent computation to be performed in task pool as a
  future

## std::future::Future

* Odd Rust implementation of future idea

* New trait

        pub trait Future {
            type Output;
            fn poll(
                self: Pin<&mut Self>,
                cx: &mut Context,
            ) -> Poll<Self::Output>;
        }

* Ignore the `Pin` and `Context`

* When `poll()` is called on a `Future`, it will
  return either `Poll::Pending` or `Poll::Ready(val)`

* If `poll()` returns `Poll::Pending`, it will also
  arrange for the future to wake up *via* callback
  when it might be able to proceed with the computation

## async/await

* Can declare a function or block `async` to get an "async context"

* The compiler builds a custom implementation of `Future`
  for this block or function that "does the right thing"
  in case of `.await` or returning

* If the result type of the function or block is declared as
  `T` it is actually `impl Future<Output=T>`

* If you evaluate such a function or block with `.await`, it
  will return a future that polls until it gets a ready
  value and then it will return that value

* You cannot call async functions outside of an async
  context

## Rust's async/await Ecosystem

* Need a few things:

    * Implementations of potentially-blocking primitives
      that are `async`

    * Task pool implementation that provides machinery for
      spawning and supplying worker threads

* Currently provided by either the `tokio` ecosystem or the
  `async-std` ecosystem or `smol` or many other options

* Idea of `std::future::Future` is that you can use the
  primitives with either ecosystem (or build your own
  machinery, or whatever)

## Pick Your Ecosystem

* Tokio

    * Older: lots more experience in community (through
      versions 0.1, 0.2, 0.3, 1.0)
    * Firefox thing
    * Full of subtle traps

* async-std

    * Easier to get started with
        * Slightly cleaner interface
        * Better comparability with std
    * Now mostly abandoned

* smol
    * Underlies async-std
    * Small
    * Good option for simple things

## Using std::future and the futures Crate

* Some interfaces are sort of runtime-agnostic through these

* Better to use these generic interfaces when you can

## async/await Maturity

* All of this is a bit immature (after many years)

* Still soundness issues

* The
  [Colored Function Problem](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/)
  is real

* Missing async traits are a big deal; probably coming soon

## Example

<http://github.com/pdx-cs-rust/countserver>

## Concurrent vs Parallel

* Concurrent: things *could be* / *appear to be* parallel

* Parallel: things are happening at the same time

* Note that parallel ⇒ concurrent but not vice-versa

## Why Parallel?

* Used to be hot topic. Meh

* CPUs/memory aren't getting faster, just more parallel

* Big Data / Machine Learning etc

## Why Concurrent?

* Some things are naturally concurrent:

  * User input
  * Agent-based systems

## Why Not Parallel?

* Only gives linear speedups

* Generally harder to get right

## Rust's Parallel Story

* Eliminates shared-memory errors

* Eliminates data races

* No significant overhead

* Parallel systems are still hard: partitioning, deadlocks,
  termination, etc

## Approaches To Parallelism

* "Background threads"

* Worker pool

* Pipeline

* Data parallel

* "Sea of synchronized objects"

## Thread

* Thread of control, with own registers and stack

* Shares its memory with all other threads in its process

* Rust uses "OS threads"

* (Other languages sometimes use "green threads")

## Approaches To Parallel Comms / Sync

                 c = 15
      acquire(m)        acquire(m)
      r = load(c)   |   r = load(c)
      r += 1        |   r += 1
      store(r, c)   |   store(r, c)
      release(m)        release(m)
      
                 c = 17

                      c = 15
      fetch_add(c, 1)   |   fetch_add(c, 1)
                      c = 17

* CPU-atomic operations

* Mutexes, semaphores, etc

* Channels

                 c = 15
          t                t'
        recv(m)    |   send(m, t)
        c += 1

## Thread Fork/Join, Shared Memory

* The "old-school" approach

* <http://github.com/pdx-cs-rust/parallel>

* Rust's error handling is nice here.

## Getting Memory To Threads

* Can use `move` closure to move data into thread

* Can use `Arc` to share data across threads

## Rayon

* Uses "map-reduce" paradigm with worker pool

* The demo here is not too good a match to Rayon's use case

## Channels

* Nice way to get values sent for long-running threads

* `mpsc`: allows building DAGs of channels

* Need `mpmc` to distribute messages among threads

* Data is actually moved or shared across channels;
  efficient

## Synchronization

* `Mutex<T>`: Wraps a value in a mutex lock

  * For example `mutex = Mutex::new(0_usize)`
  * Only one thread may complete `mutex.lock()` at a time
  * That thread gets a mutable "guard" for the wrapped data
  * Other threads are blocked until first releases lock
  * Returns a `Result` because mutex poisoning
  * For example `*mutex.lock().unwrap() = 5`

* `RwLock<T>`: A mutex that will give out many shared
  immutable refs or one mutable ref at a time

* `AtomicUsize` *et al*: Ensures that read-modify-write
  operations happen as an atomic thing
  
  * Provides methods like `.fetch_add(1, Ordering::SeqCst)`
  
  * The "memory ordering" should probably just always be set
    to that

  * Not as great as they sound

* `Condvar`: Block waiting for an event

  * Super-complicated, involving `Mutex` plus validation state
  * See the docs for details

## Send + Sync

* Marker traits for thread safety

* Automatically maintained (for the most part) by the
  compiler
  
* `Send`: Safe to move value to another thread

* `Sync`: Safe to share a non-mut reference to value with
  another thread

* `Send` and `Sync` are auto-derived for structs / enums

## Pipeline Parallelism

* Easy-to-get-right thing; used in CPUs a lot

* A bit tricky to set up

* Only gives parallelism with multiple pipeline operations

* Nice book example of iterator construction for pipelining

  
## Polling Loops

* What *not* to do: spin repeatedly checking for a thing

    * Burns CPU for no reason
    * Sure, timeouts, but impossible to choose well
    * Use blocking primitives

## Book Misc

* MPMC (with a funky implementation)

* Globals and "thread-locals"

* Mandelbrot

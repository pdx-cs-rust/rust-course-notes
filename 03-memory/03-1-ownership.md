## Standard Memory Management

* Automatic: Garbage collector or reference counting
  system

  * Garbage Collector: When low on memory, trace out all
    accessible memory, free non-accessible

  * Reference Counting: Keep track of how many references
    to a particular chunk of memory. When count goes to
    zero, free it

* Manual: Programmer keeps track of which memory should be
  preserved, allocates new memory, frees old

## Rust Memory Management

* Invisible "Manual": Compiler issues code to allocate memory
  and free memory where needed

* This is restrictive: programmer must ensure that memory is
  not allocated too late or freed to early, in the presence
  of pointers

* Rust compiler ensures that memory is allocated before use,
  statically unavailable at time of free
  
* Key is lexical scope: when a variable statically leaves
  scope, its value is no longer reachable, so freed

## Memory Allocation

* By default, a value must be allocated in memory unless it
  is small enough to fit in a register and marked copyable
  and bleah bleah bleah
  
* Choices are stack allocation or heap allocation: default
  is stack

* Heap allocation is ultimately done in `unsafe` code

* `Drop` trait allows explicit actions during deallocation

## Copyable Values

* If a type has the `Copy` trait (e.g. the integer types)
  the compiler will feel free to make a copy of it whenever
  convenient

v* If a type has the `Clone` trait (e.g. most built-in types)
  the compiler will make a (deep) copy whenever the type's
  `clone()` method is called

* Otherwise there will be no user-visible copying

## Moves

* The compiler may choose to insert code to move a thing to
  a different place in memory
  
* If this happens, it will not make a copy: it will leave
  the old thing uninitialized and unreferenceable.
  
## Failing Move

    #[derive(Debug)]
    struct Bogus;

    fn main() {
        let x = Bogus;
        let y = x;
        println!("{:?}", y);
        println!("{:?}", x);
    }

## Ownership

* Net effect of all this: at any given time a value is
  "owned" by a particular name

* The value is given to the owning name when it is created
  (Resource Acquisition Is Initialization = RAII)

* The value is freed when the owning name leaves scope

* Ownership can be transferred by a move

## The Takeaway

* Need to develop an operational mental model of ownership

* When confused, refer to that model or get help

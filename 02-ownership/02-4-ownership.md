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

* The compiler uses ownership rules to determine when values
  are dropped

* This is restrictive: programmer must ensure that memory is
  not allocated too late or freed too early, in the presence
  of pointers

* Rust compiler ensures that values cannot be used before
  initialization or after they have been moved or dropped
  
* Key is lexical scope: when an owner leaves scope, its value
  is dropped unless ownership has been transferred

## Memory Allocation

* Local values are conceptually stored in their stack frames.
  The compiler may keep them in registers or optimize them away
  
* Choices are stack allocation or heap allocation: default
  is stack

* Heap allocation is ultimately done in `unsafe` code

* `Drop` trait allows explicit actions during deallocation

## Copyable Values

* If a type has the `Copy` trait (e.g. the integer types),
  assignment and argument passing copy the value, leaving the
  original usable

* If a type has the `Clone` trait (e.g. most built-in types)
  it provides an explicit `clone()` method. A clone should
  duplicate the value's data up to references, but may share
  underlying referenced data

* Otherwise assignment and argument passing move ownership,
  leaving the original binding unusable

## Moves

* A move transfers ownership from one place to another
  
* The old place is left uninitialized and cannot be used

* This is a language-level rule. It does not imply that the
  compiler physically relocates the value
  
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

* Net effect of all this: at any given time a value has one
  owner

* A value gets an owner when it is created
  (Resource Acquisition Is Initialization = RAII)

* The value is dropped when its owner leaves scope, unless
  ownership has been transferred

* Ownership can be transferred by a move

## The Takeaway

* Need to develop an operational mental model of ownership

* When confused, refer to that model or get help

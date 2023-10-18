## Deref, DerefMut

* Used to transfer a `*` dereference of a type to some
  type in the inner structure
  
* Canonical cases are `Box`, `Vec`, `String` and similar containers

## AsRef, AsMut

* Annoying Rust shorthand: "refmut" is usually just spelled
  "mut"

* Traits for borrowing a reference to your type from a
  variety of values

* Usually used for generic parameters. Book says

          fn open<P: AsRef<Path>>(path: P) -> ... {
             let path: &Path = path.as_ref();
             ...

          open("foo.txt".to_string())

## Borrow, BorrowMut

* Almost identical to `AsRef`, `AsMut` ?!?

* By convention, implement for a type only when references
  are semi-interchangeable with values

* Mostly for collection type convenience

* This is where the types start to get truly ugly: see the
  book example

## From, Into

* Type conversion traits

* `Into` is just `From` with the types reversed

* Normally just implement `From` to get a default
  implementation of `Into`

* `from()` and `into()` consume their arguments!

* `from()` and `into()` should never fail!

## TryFrom, TryInto

* Like `From` and `Into` but return a `Result` instead of
  panicking

* Makes it possible to safely convert types when some of the
  values may not safely convert

## ToOwned

* Trait for "cloning" a thing that implements the `Borrow`
  trait

* One standard way to create a `String` from an `&str`:

          "hello".to_owned()

* Also works for slice to `Vec`

## Cow

* "Copy On Write": keeps a reference until owned

* Glory in the beauty of this: book says

          enum Cow<'a, B: ?Sized + 'a>
               where B: ToOwned
          {
              Borrowed(&'a B),
              Owned(<B as ToOwned>::Owned),
          }

* We can do

          fn f<'a>(...) -> Cow<'a, str> {
          }

## Observations

* There's a lot of stuff here

* Read this chapter carefully, then…

* Return to it as you advance in Rust. The details are best
  appreciated in the context of a problem you are trying to
  solve

* The result of this mess is a pretty expressive language:
  much is "hidden under the hood"

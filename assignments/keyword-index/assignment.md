It is sometimes useful to hold references into parts of a
data structure. In this exercise, we will focus on maintaining an
index of words in texts.

## Assignment

You will be processing the words in a collection of
documents stored in-memory. You could make an owned list of
all the words you find, but that would more than double the
memory. Instead, you cook up a struct to store each word
as a slice into its original text — a "keyword index".

```rust
/// Each slice in this struct's list is a word in some
/// in-memory text document.
#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);
```

You now want to provide an interface that implements this
struct's functionality. You must create a new library crate
`kwindex` and implement `KWIndex` with the API below. (You
do not need to copy the doc comments in your API. However,
there are some basic doctests in the examples that you might
want.)

```rust
impl<'a> KWIndex<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        todo!()
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this `KWIndex`
    /// index.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up an index.
    ///
    /// Words are separated by whitespace or punctuation,
    /// and consist of a span of one or more consecutive
    /// letters (any UTF-8 character in the "letter" class)
    /// with no internal punctuation.
    ///
    /// For example, the text
    ///
    /// ```text
    /// "It ain't over untïl it ain't, over."
    /// ```
    ///
    /// contains the sequence of words `"It"`, `"over"`,
    /// `"untïl"`, `"it"`, `"over"`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("Hello world.");
    /// assert_eq!(2, kwindex.len());
    /// assert_eq!(1, kwindex.count_matches("world"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        todo!()
    }

    /// Count the number of occurrences of the given `keyword`
    /// that are indexed by this `KWIndex`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("b b b-banana b");
    /// assert_eq!(3, kwindex.count_matches("b"));
    /// ```
    pub fn count_matches(&self, keyword: &str) -> usize {
        todo!()
    }

    /// Return the *n*-th uppercase word (all characters are
    /// Unicode uppercase, *n*-th counting from 0) that is indexed
    /// by this `KWIndex`, if any.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("I am THE WALRUS");
    /// assert_eq!(Some("THE"), kwindex.nth_uppercase(1));
    /// ```
    pub fn nth_uppercase(&self, n: usize) -> Option<&str> {
        todo!()
    }

    /// Count the number of words that are indexed by this
    /// `KWIndex`.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use kwindex::KWIndex;
    /// let kwindex = KWIndex::new()
    ///     .extend_from_text("Can't stop this!");
    /// assert_eq!(2, kwindex.len());
    /// ```
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Is this index empty?
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
```

Implement each of these methods. You must then write some
basic tests.

You *must not* be cloning or otherwise duplicating strings
in your library.

## Hints

* You may struggle a bit with reference types and lifetimes
  in this assignment. That's normal.

* Things I used in my implementation (in no particular order):

    * `s.chars()` is an iterator that returns each character
      in a string slice `s` in turn.

    * `s.split_whitespace()` is an iterator that returns
      each slice of non-whitespace characters in a string
      slice `s` in turn.

    * `c.is_alphabetic()` returns true iff the character `c`
      is alphabetic (a "letter").

    * `c.is_uppercase()` returns true iff the character `c`
      is uppercase (an "uppercase letter").

    * `s.trim_matches(|c: char| ...)` is a bit tricky. This method
       takes a boolean function ("closure" / "lambda") that
       should return `true` iff the given character should
       be trimmed. The method returns a subslice of the
       string slice `s` that has matching characters trimmed
       off the start and end of the string.

* If you are feeling functional-programmy, check out the
  `map()` and `filter()` operations on iterators, which can
  be pretty useful.

## Requirements

Your library crate implementation should contain adequate
tests (implemented using `#[test]` unit-testing) and
assertions (implemented using `assert!()` and related
macros). Your code should be formatted according to the
official Rust formatting style — use `cargo fmt` to reformat
your code in-place. Your code should produce no compiler
warnings, and `cargo clippy` should also produce no
warnings. Please do not disable warnings except in the most
unusual circumstances: fix them instead.

Please submit a recursive ZIP archive of a single directory
`kwindex` containing:

* Your `Cargo.toml`.

* Your `src/lib.rs`.

* Any other source or other text files that are
  necessary/useful.

* A `README.md` file in Markdown format giving a writeup of
  what you did, how it went, and how you tested your work.

* Nothing else. Not your git repo. None of the funny Mac
  garbage files. Not your `target/` directory.

//! Big Bag Of Words
//!
//! The "Big Bag Of Words" is used in text analysis and
//! machine learning.  It reduces a text to a collection of
//! words, each with a count of the number of occurrences.
//!
//! This implementation uses zero-copy strings when
//! reasonably possible to improve performance and reduce
//! memory usage.
//!
//! Words are separated by whitespace, and consist of a
//! span of one or more consecutive letters (any Unicode
//! code point in the "letter" class) with no internal
//! punctuation: leading and trailing punctuation are
//! removed.
//!
//! For example, the text
//!
//! ```text
//! "It ain't over untïl it ain't, over."
//! ```
//!
//! contains the sequence of words `"It"`, `"over"`,
//! `"untïl"`, `"it"`, `"over"`.
//!
//! Words in the bag containing uppercase letters will be
//! represented by their lowercase equivalent.

use std::borrow::Cow;
use std::collections::BTreeMap;

/// Each key in this struct's map is a word in some
/// in-memory text document. The corresponding value is the
/// count of occurrences.
#[derive(Debug, Default, Clone)]
pub struct Bbow<'a>(BTreeMap<Cow<'a, str>, usize>);

fn is_word(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_alphabetic())
}

fn has_uppercase(word: &str) -> bool {
    word.chars().any(char::is_uppercase)
}

impl<'a> Bbow<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this BBOW.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up a BBOW covering
    /// multiple texts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new().extend_from_text("Hello world.");
    /// assert_eq!(2, bbow.len());
    /// assert_eq!(1, bbow.match_count("hello"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        todo!();

        self
    }

    /// Report the number of occurrences of the given
    /// `keyword` that are indexed by this BBOW. The keyword
    /// should be lowercase and not contain punctuation, as
    /// per the rules of BBOW: otherwise the keyword will
    /// not match and 0 will be returned.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("b b b-banana b");
    /// assert_eq!(3, bbow.match_count("b"));
    /// ```
    pub fn match_count(&self, keyword: &str) -> usize {
        todo!()
    }

    pub fn words(&'a self) -> impl Iterator<Item=&'a str> {
        self.0.keys().map(|w| w.as_ref())
    }

    /// Count the overall number of words contained in this BBOW:
    /// multiple occurrences are considered separate.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(3, bbow.count());
    /// ```
    pub fn count(&self) -> usize {
        todo!()
    }

    /// Count the number of unique words contained in this BBOW,
    /// not considering number of occurrences.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(2, bbow.len());
    /// ```
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Is this BBOW empty?
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

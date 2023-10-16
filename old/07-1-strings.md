## Strings (ugh)

* Lots of stuff here

* Fortunately, we've talked about a lot of it before

* tl;dr:

  * There's `char` which is a Unicode "code point"
  * There's `str` which is a sequence of bytes representing
    Unicode code points in the compressed UTF-8 format
  * There's `&str` which is a smart pointer to a `str` with
    with a length field
  * There's `String` which is the "owned" version of `str`
  
## Chars

* "Character" can mean a lot of things. There's 7-bit ASCII,
  8-bit "latin-1". There's a million per-language coding
  standards
   
* In many languages there can be some debate about what
  constitutes a single "character". Even in English, is
  a ligature like `ﬀ` a character?

* Rust's `char` is a Unicode "code point". It's a 32-bit
  quantity, but not all possible values are legal
  
## Char Features

* There are the usual character classifiers and converters,
  which work on full Unicode
  
* There's a bit of ASCII support, but probably shouldn't
  normally use it

* You can always cast a `char` to any integer type. Watch
  out for overflow!

## Char Conversions

* You can't cast an integer to `char` (except `u8`, because
  ASCII): you need to use `std::char::from_u32()` or
  something like it. It returns an `Option` depending on
  whether the particular input is a legal Unicode code point

* Note that case conversions can't return a single character
  because uppercase ←→ lowercase is not always 1::1. So
  return a `char` iterator, which is super-annoying

## Char Examples

* `examples/charcast.rs`

* `examples/lower.rs`

## str

* It is deemed "not best practice" to store strings as
  sequences of 32-bit code points. So a compressed encoding
  called UTF-8 is used for strings. This encoding stores
  ASCII characters as themselves, and uses an escape
  convention to get multibyte coding of non-ASCII strings.
  A UTF-8 string is almost always much smaller than 4x the
  number of code points

* A Rust `str` is like an array, except of UTF-8 text. A
  `str` is unsized, so it is really only useful in certain
  type declarations
  
## String and &str

* Let's just refer to `&str` and `String` values
  collectively as "strings"

* An `&str` is a reference to a `str`. It is a fat pointer
  that contains the size of the `&str` in bytes. The normal
  borrow rules apply

* A `String` is an owned reference to a `str`. It is a
  fat pointer that contains the size of the `str` in bytes.

* Because `String` is owned, you can modify the contained
  bytes. However, all the methods provided for this are
  guaranteed to preserve UTF-8 encoding of the bytes. This
  is carefully tuned to avoid trouble
  
## String Features

* You can get an iterator over a string's code points
  (`.chars()`) or `u8` bytes (`.bytes()`)

* There is no convenient way to go to a given character
  (code point) position in a string. If you plan to do that
  a lot, use `chars().collect()`

* `examples/stringchars.rs`

## String Methods

* There are the obvious methods for converting these things
  around. Read the book carefully to learn about the
  vocabulary
  
* Many of the string manipulation functions take a "pattern".
  There's a lot of kinds: read the book for the
  details. You will use them all, eventually

## String Conversions

* You can use `from_str()` or `.parse()` to convert a string
  into something else. You can use `to_string()` to convert
  other things into `String`
  
* You can use `.as_bytes()` and `.into_bytes()` to grab
  the bytes of a string for free
  
* The `::from_utf8()` methods come in checked, lossy and
  unsafe flavors. Choose wisely

* There's a regex library crate. It's good.

## Cow

* Previously discussed. Allows avoiding conversion until
  needed

* Usually not worth the trouble

## Formatting

* The book discusses the details of the format string
  language we've been using with `println!()` and the
  like. Read it

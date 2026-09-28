# HW: sublists_pairs

Sometimes you need to deal with less-than-ideal input. In this
assignment, someone is going to pass you a list of lists of
values. The values in the sublists are treated as “paired up” —
the sublists should always be of even length, otherwise it’s an
input error. Your job is to iterate over this list of lists and
produce pairs of values (in order) from the sublists.

## Assignment

In this assignment you will be finishing the implementation of
`sublists_pairs()`.

To do the assignment, clone
<https://github.com/pdx-cs-rust/hw-sublists_pairs>. Then follow
the instructions there.

## Hints

- Don’t be afraid to ask for help/clarification on Zulip.

- Write some of your own tests for these: testing is everything
  in this situation.

- There’s a solution in just a few lines of code if you use
  closures wisely with `std::iter` stuff. Otherwise your solution
  will be a lot more verbose. You do not need a separate struct
  for your iterator, but it may be helpful.

## Requirements

Your library crate implementation should contain adequate tests
(implemented using `#[test]` unit-testing) and assertions
(implemented using `assert!()` and related macros). Your code
should be formatted according to the official Rust formatting
style — use `cargo fmt` to reformat your code in-place. Your code
should produce no compiler warnings, and `cargo clippy` should
also produce no warnings. Please do not disable warnings except
in the most unusual circumstances: fix them instead.

Please submit a recursive ZIP archive of a single directory
`hw-sublists_pairs` containing your completed version of the
assignment. Don’t forget to *not* submit your `.git` Git repo:
just submit the source files.

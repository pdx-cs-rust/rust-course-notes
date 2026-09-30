# HW: Cache

Caches are one of the most important tools of systems engineers.
A cache attempts to utilize small but fast storage to hold a set
of values that will likely be needed in the future. The cache
thus minimizes the cost of re-fetching or re-computing values.

There are many ways to implement a cache. The cache contents
might be almost anything. Standard cache *policies* — decision
procedures dictating which values stay in the cache when space is
full — are many and varied.

From the client’s perspective, a cache is a “closed box” with a
simple interface. Given various cache implementations, the client
can choose the one that best suits their needs.

## Assignment

In this assignment you will be finishing the implementation of
various caches in Rust. All of these implementations conform to a
common `Cache` trait: see <https://github.com/pdx-cs-rust/cache>.

To do the assignment, clone
<https://github.com/pdx-cs-rust/hw-cache>. Then follow the
instructions there.

## Hints

- Don’t be afraid to ask for help/clarification on Zulip.

- Write some of your own tests for these: testing is everything
  in this situation.

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
`hw-cache` containing your completed version of the assignment.
Don’t forget to not submit your `.git` Git repo: just submit the
source files.

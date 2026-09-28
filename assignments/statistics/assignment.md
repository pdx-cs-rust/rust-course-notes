In this assignment, you will set up a Rust project, then add
to it some simple statistical computations to complete a
working library and application.

1. (If you do not already have one.) Make an account on
   <http://github.com>.  Follow the instructions there to
   set the account up so that you can push to it.

2. Go to <http://github.com/pdx-cs-rust/stats> and fork it.
   (Press the "Fork" button while logged into Github.)  Use
   the Github interface to mark your repository "private" so
   that others cannot "share" your work.

3. Clone your new repository locally so that you can work on
   it.

4. Examine the new project carefully; this is the basic
   format your projects should all follow in this course.

   Note the `LICENSE` file and `README.md`. Edit these to
   contain your name and copyright notice instead of
   mine. Add your `pdx.edu` email address to `README.md`.

   Note that the project as checked in contains neither a
   `Cargo.lock` file nor a `target/` subdirectory. This is
   good practice.

5. (If you have not already done so.) Set up Rust using
   [rustup](http://rustup.rs) for your target machine.
   Be sure to get your Linux "path" (or equivalent Windows
   or Mac thing) to include the location of `rustc` etc.
   Test this by running `rustc --version`. (The version as
   of this writing should be `1.35.0`: later is fine.)

   Note that you will also need to follow these steps for
   PSU Linux machines. These machines typically have either
   no installation of Rust or a badly out-of-date one: you
   want to install your own.

6. Run these commands to add important functionality to your
   Rust installation:

           rustup component add rustfmt
           rustup component add clippy

7. Examine `src/lib.rs`. This is the library you are asked
   to complete in this assignment. Find the
   `unimplemented!()` expressions in the code and replace
   them with code to compute the appropriate stats.
   You may need to look up the relevant stats with a Google
   search to refresh your memory.

   Run `cargo build` to compile your new stuff, and `cargo
   test` to test it. Make sure to `git commit` frequently
   (with meaningful commit messages) to preserve the state
   of your work. Repeat until your code compiles without
   errors or warnings and passes all tests.

8. Add at least one test of your own to each of the library
   functions. You may do this as a doctest (in the style
   already show) or with separate functions marked `#[test]`
   if you feel that your tests do not belong in the
   documentation. See the Rust Book
   [Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
   chapter for further information on testing. Make sure
   your new tests also pass.

9. Make sure your work is committed to your repository. (Use
   `git status` to see what is up.) You should not have
   committed `Cargo.lock` or the `target/` subdirectory.

10. Run `cargo fmt` and then `git diff` to see what the
    auto-formatter has done to your code. Edit as you see
    fit (but `cargo fmt` usually makes good suggestions) and
    commit the result.

11. Run `cargo clippy` and see what suggestions it makes
    about your code. Edit as you see fit (but `cargo clippy`
    usually makes good suggestions) and commit the result.

12. Push your work back to Github with `git push`.

13. Make sure that Github user `BartMassey` has read access
    to your Github repository. Otherwise, since your
    repository is private, I won't be able to see it.

14. Submit your Github repository URL (*and only the URL*)
    to the course Moodle as your assignment solution.

## Hints

* Numeric literals may have a type suffix, for example
  `0usize`. Underbars inside numeric literals are ignored,
  which is useful for formatting: for example `0_usize`.
  Floating literals must have a decimal point: for example
  `9.0`.

* You can convert an integer to `f64` with `as`: for
  example, `7 as f64`.

* Functions like `sqrt()` and `powf()` can be called with
  object-style syntax: for example, `9.0f64.sqrt()`. They
  can also be called as functions of the given type: for
  example `f64::sqrt(9.0)`.

* An `Option` type is a type that wraps up a value. Possible
  values of type `Option<f64>` include `None`, `Some(0.0)`
  and other `Some(x)` values where `x` is a floating-point
  number. See the Rust manual or the book for a discussion
  of `Option`.

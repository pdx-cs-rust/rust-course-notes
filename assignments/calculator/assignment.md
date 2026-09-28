Implement the command-line `gcd` example of Chapter 2 of the
course text. Rename it as `calc`. Modify it to apply
different functions depending on the first argument passed
to it. Functions to be implemented:

* "sum": The sum of the remaining arguments.

          $ target/debug/calc sum 1 2 3
          6

* "product": The product of the remaining arguments.

          $ target/debug/calc product 1 2 3
          6

* "gcd": The Greatest Common Divisor of the remaining arguments.

          $ target/debug/calc gcd 10 14 12
          2

* "lcm": The Least Common Multiple of the remaining arguments.

          $ target/debug/calc lcm 10 14 12
          420

When no arguments beyond the function are given, `calc`
should silently exit with success.

          $ target/debug/calc lcm
          $ echo $?
          0

When one argument beyond the function is given, `calc`
should display that argument and exit with success.

          $ target/debug/calc lcm 12
          12

Your `calc` implementation should contain adequate tests and
assertions. It should be formatted according to the official
Rust formatting style.

Submit your files as a ZIP archive named
`hw1-`*mylastname*`.zip`.  Your submission should contain
your source file(s) and `Cargo.toml`: it should *not*
contain a `target/` directory or `.git` directory. Your
submission should contain a `README.md` file in Markdown
format giving a writeup of what you did, how it went, and
how you tested it.

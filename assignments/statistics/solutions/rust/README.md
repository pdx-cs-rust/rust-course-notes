# Stats
Copyright (c) 2019 Bart Massey

This program computes a basic statistic. Its input is
taken from `stdin`, and must consist of floating-point
numbers as text, one per line.

The program outputs one of the following statistics as the
text of a floating-point number on stdout.

* `--mean`: Arithmetic Mean
* `--stddev`: Population Standard Deviation
* `--median`: Median
* `--l2`: Euclidean Norm

The various statistics are implemented in the `stats`
library crate, which can be used by other programs as well.

## Build and Run

Build this program and library with `cargo build`. You can
run the program with `cargo run`. You will need to pass a
`--` before the program flag: for example,

    cargo run -- --mean

To build or run an optimized version, use `cargo --release`.

Run `cargo test` to do some simple testing.

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.

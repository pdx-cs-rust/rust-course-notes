# rule110: CA Rule 110 runner
Bart Massey 2025

This is a solution to
<https://github.com/pdx-cs-rust/hw-rule110>.  This version
is specifically Rule 110 and uses bit-parallelism to be
quite fast.

Say `cargo run --help` for information on command-line
arguments.

## Design

This is intentionally a generalized challenge solution, not a
model of the simplest solution expected for the assignment. It
accepts starting rows from 1 through 64 cells, supports a
configurable number of rows, and can print just the last row. Its
`u64` representation updates every cell in a row in parallel.

The straightforward assignment solution should probably use
`[bool; 8]` and update one cell at a time.

## Future Work

Add loop detection and reporting.

## License

This work is made available under the "Apache 2.0 or MIT
License". See the file `LICENSE.txt` in this distribution for
license terms.

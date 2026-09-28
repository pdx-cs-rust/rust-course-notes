# modexp: Modular Exponentiation Calculator
Bart Massey

This program computes the value of `x**y mod m` where `**`
is exponentiation. The arguments must be small integers: `x`
and `y` less than `2**32`, `m` less than `2**64`. All
arguments must be non-negative: `m` must be greater than 0.

## Build and Run

Install a working Rust environment and say `cargo run 2 31 2`
in the source directory. The program should print `0`.

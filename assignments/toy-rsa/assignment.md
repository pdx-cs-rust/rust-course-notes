## Background

"Public key cryptography" is super-important in the 21st century. The
basic idea is an encryption scheme with two "keys": a "public key" used
for encryption that can be shared with anyone, and a corresponding
"private key" that is the only way to decrypt. Thus, if Alice wants to
send a secret message *M* to Bob, Alice can

1.  Look up Bob's public key *Kpub* in a public key directory.
2.  Encrypt *M* using *Kpub* to get a ciphertext *C*.
3.  Send *C* to Bob in public.

Bob can then decrypt *C* using his corresponding private key *Kpriv* to
get *M*. No one else has *Kpriv*, so no one else can decrypt *C*.

Normally, because reasons, the message sent by Alice will itself be just
a number *Ksymm* that is the key in some "symmetric" cipher that Alice
and Bob will switch to for future communications.

[RSA](https://en.wikipedia.org/wiki/RSA_%28cryptosystem%29) (named for
its creators Rivest, Shamir and Adleman) is the earliest public-key
cryptosystem. It is still widely used today.

## Assignment

In this assignment, you will write a library crate that provides RSA key
generation, encryption and decryption. I cannot emphasize enough that
**this RSA crate will be a toy exercise, and should not be used for
anything that needs to be kept secure in real-life situations.**

Start by carefully reading the Wikipedia page linked in the previous
section. There's a lot of number theory there, and you are not expected
to understand that part. You are looking for the algorithms and
pseudocode used for RSA.

Create a library crate called "toy-rsa" (the crate name is in the
`Cargo.toml`. This crate should provide the following interface:

        /// Fixed RSA encryption exponent.
        pub const EXP: u64 = 65_537;

        /// Generate a pair of primes in the range `2**31..2**32`
        /// suitable for RSA encryption with exponent.
        pub fn genkey() -> (u32, u32)

        /// Encrypt the plaintext `msg` using the RSA public `key`
        /// and return the ciphertext.
        pub fn encrypt(key: u64, msg: u32) -> u64

        /// Decrypt the cipertext `msg` using the RSA private `key`
        /// and return the resulting plaintext.
        pub fn decrypt(key: (u32, u32), msg: u64) -> u32

(Note that, as explained in the **Background** section above, the
plaintext `msg` is just a 32-bit unsigned integer. No strings are
involved in this assignment.)

Here is pseudocode for the implementation. Consult the linked Wikipedia
page for the story behind this pseudocode.

> *E* = 65537
>
> 𝜆(*p*, *q*):\
>     **return** least common multiple of *p* - 1 and *q* - 1
>
> *encrypt*(*key*, *msg*):\
>     **return** *msg*^*E*^ **mod** *key*
>
> *decrypt*(*key* = *p* ⋅ *q*, *msg*):\
>     d ← inverse of *E* **mod** 𝜆(*p*, *q*)\
>     **return** *msg*^d^ **mod** (*p* ⋅ *q*)
>
> *genkey*:\
>     **repeat** \
>         *p*, *q* ← rsa primes (primes in range 2^31^ .. 2^32^-1)\
>     **until** *E* \< 𝜆(*p*, *q*) **and** **gcd**(*E*, 𝜆(*p*, *q*)) = 1\
>     **return** *p*, *q*

The functions you will need to implement this are already provided for
you by a library crate that I wrote. In your `Cargo.toml`, put

    [dependencies.toy_rsa_lib]
    git = "http://github.com/pdx-cs-rust/toy-rsa-lib"

You can then `use toy_rsa_lib::*` in your `src/lib.rs` to get these
functions. See the [toy-rsa-lib
rustdoc](https://pdx-cs-rust.github.io/toy-rsa-lib/toy_rsa_lib/index.html)
for a summary of my crate.

## Hints

-   All the issues with numeric type conversions are present here. See
    the course video on type conversions for more info.

-   I would suggest putting some randomized testing together --- this is
    what I did. You can make a `tests/` directory in your project and
    write your tests there to keep things cleaner. You still write test
    functions and tag them with `#[test]` in this case --- I wrote
    `tests/random.rs`.

-   I would suggest writing a command-line program to try your work
    (this is what I did). You can make an `examples/` directory in your
    project and write your program there --- I wrote
    `examples/toyrsa.rs`. You can run your program with
    `cargo run --example` --- I used `cargo run --example toyrsa`.

-   Here's a test case to get started with:

          Private Key: p = 0xed23e6cd q = 0xf050a04d
          Public Key: p * q = 0xde9c5816141c8ba9
          Message: 0x12345f
          Encrypted: 0x6418280e0c4d7675
          Decrypted: 0x12345f

## Requirements

-   Your library must build with current `stable` Rust.

-   Your library implementations should contain adequate tests
    (implemented using `#[test]` unit-testing) and assertions
    (implemented using `assert!()` and related macros).

-   Your code must be formatted according to the official Rust
    formatting style --- use `cargo fmt` to reformat your code in-place.

-   Your code must produce no compiler warnings, and `cargo clippy` must
    also produce no warnings. Please do not disable warnings except in
    the most unusual circumstances: fix them instead.

Please submit a ZIP archive containing:

-   Your `Cargo.toml` and `Cargo.lock`.

-   Your `src/lib.rs`.

-   Any other source or other text files that are necessary/useful.

-   A `README.md` file in Markdown format giving a writeup of what you
    did, how it went, and how you tested your work.

-   Nothing else. Not your git repo. None of the funny Mac garbage
    files. Not your `target/` directory.

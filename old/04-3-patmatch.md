## Pattern Matching

* Not specific to structs / enums

* Mechanism for deconstructing a composite thing

        let t = (3u8, "hello");
        let (x, h) = t;
        println!("{} {}", x, h);

* We've seen some of this before.

* Two kinds: "refutable" and "irrefutable"

  * `let` requires irrefutable pattern
  * `match` allows refutable pattern, but
    all possible patterns must be matched

## Match

* The `match` statement is the canonical pattern matching
  tool
  
          struct Point {
              x: i64,
              y: i64,
          }

          let p = Point{x: 3, y: 5};
          let z = match p {
              Point{x: 3, y: 5} => {
                  println!("is good");
                  8
              }
              _ => {
                  panic!("bad point");
              }
          };
          match p {
              Point{x: xx@1..=3, ..} => println!("{}", xx),
              Point{x: xx@0, y: 4} => println!("{}", xx),
              _ => panic!("bad point"),
          }
  
* Note the existence of `..`, `..=`, `|` (not shown, only at top level), `@`

* First match is chosen

* Compiler checks that every possible value will be matched,
  and that later matches are not subsumed by earlier
  
## Pattern Variable Modifiers

* Rules for move/borrow are the same for pattern variables

* For convenience, you can modify a pattern variable with
  `ref` or `ref mut` to have it borrowed instead of moved

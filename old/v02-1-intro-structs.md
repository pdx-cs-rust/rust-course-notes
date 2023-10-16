## Data Structuring

* Structure types e.g. `struct Name { x: u8, y: char }`

* Enumerated types e.g. `enum Name { X, Y(char) }`

## Structs

* Fairly standard structure syntax with colons

          struct Point {
              x: u64,
              y: u64,
          }

  initialized as they look

          let p = Point { x: 12, y: 20 };

## Impl Struct

* Structs can have an "implementation"

          impl Point {
              fn x_coord(&self) -> u64 {
                  self.x
              }
          }

  for OO-like syntax and vaguely OO semantics

          let px = p.x_coord()

* "Tuple structs" are a thing:

          struct Point(u64, u64);

## Enums

* Fairly simple

          enum Color {
              Red,
              Green,
              Blue,
              Custom(f64, f64, f64),
          }

* "Disjoint union" / "sum type": value of this type is one
  of the above. Note that variants can contain values

          let c = Color::Custom(0.7,0.5,0.3)


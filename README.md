# NOTICE

The library deprecated for now,
until i find a time to write with the following features:

- generics
- traits
- better error handling
- following rust standards

# modules coord2d

[![Crate Status](https://img.shields.io/crates/v/coord2d?style=for-the-badge)](https://crates.io/crates/coord2d)
[![docs.rs](https://img.shields.io/docsrs/coord2d?style=for-the-badge)](https://docs.rs/coord2d/0.2.7/coord2d/)
![Crates.io](https://img.shields.io/crates/l/coord2d?style=for-the-badge)

rust lib for coordinate in 2d system

## example Coord

```rust
use coord2d::*;
fn main () {
   let a: coords::Coord = new_coord!(1, 2.0);
   let o: coords::Coord = new_coord!(2.0);
   let x: i32 = 3;
   let y: f64 = 3.0;
   let b: coords::Coord = new_coord!(x, y);

   // math
   let c: coords::Coord = a + b;
   let d: coords::Coord = a - c;
   let e: coords::Coord = a * d;
   let f: coords::Coord = o / a;

   // to vec
   let g: Vec<i32> = a.to_vec();

   // to tuple
   let i: (i32, i32) = c.to_tuple();

   // split
   let (r, o) = e.to_tuple();

   println!("let a: coord::Coord = new_coord!(1, 2.0); | {:?}", a);
   println!("let o: coord::Coord = new_coord!(2.0); | {:?}", o);
   println!(
       "\nlet (x , y) = (3.0, 3);\nlet b: coords::Coord = new_coord!(x, y); | {:?}",
       b
   );
   println!("\nlet c: coord::Coord = a + b; | {:?}", c);
   println!("\nlet d: coord::Coord = a - c; | {:?}", d);
   println!("\nlet e: coord::Coord = a * d; | {:?}", e);
   println!("\nlet f: coord::Coord = a / a; | {:?}", f);
   println!("\n let g: Vec<i32> = a.to_vec(); {:?}", g);
   println!("\n let i: (i32, i32) = c.to_tuple(); | {:?}", i);
   println!("\n let (r, o) = e.to_tuple(); r={}, o={}", r, o);
}
```

## example Vectors

```rust
use coord2d::*;

fn main () {
   let a: coords::Coord = new_coord!(2.5, 3.3);
   let o: coords::Coord = new_coord!(2.5);
   let x: i32 = 3;
   let y: f32 = 3.0;
   let b: coords::Coord = new_coord!(x, y);

   let d: coords::Coord = a - b;
   let e: coords::Coord = a * d;

   let h: vectors::Vector = new_vector!();
   let f: vectors::Vector = new_vector!(o);
   let l: vectors::Vector = new_vector!(a, e);

   let m: vectors::Vector = h + l;
   let s: vectors::Vector = h + l;
   let w: vectors::Vector = m + h;
   let t: vectors::Vector = h + s;

   let vec: Vec<coords::Coord> = s.to_vec();
   let tup: (coords::Coord, coords::Coord, f64) = s.to_tuple();
   let mag: f64 = t.get_magnitude();
   let mid: coords::Coord = m.get_midpoint();
   println!("\nlet h:vectors::Vector = new_vector!(); | {:?}", h);
   println!("\nlet f:vectors::Vector = new_vector!(); | {:?}", f);
   println!("\nlet l:vectors::Vector = new_vector!(a, e); | {:?}", l);
   println!("\nlet m = h + l; | {:?}", m);
   println!("\nlet s = h + l; | {:?}", s);
   println!("\nlet w = m + h; | {:?}", w);
   println!("\nlet t = h + s; | {:?}", t);
   println!("\nlet mag = t.get_magnitude(); | {}", mag);
   println!("\nlet vec = s.to_vec(); | {:?}", vec);
   println!("\nlet tup = s.to_tuple(); | {:?}", tup);
   println!("\nlet mid: coords::Coord = m.get_midpoint(); | {:?}", mid);

}
```

## example Debug

```rust
use coord2d::*;

fn main () {
    let a: coords::Coord = new_coord!(1, 2.0);
    let x: i32 = 3.0;
    let y: i32 = 3;
    let b: coords::Coord = new_coord!(x, y);

    let l:vectors::Vector = new_vector!(a, b);

    debug::debug(a);
    debug::debug(l);

    let type_a = debug::type_of(a);
    let type_h = debug::type_of(h);

    println!("{}", type_a);
    println!("{}", type_h);

}
```

# TO DO

- [x] basic math
- [x] add fn give magnitude of vector
- [x] fix coord to accept other integers and floats types such as:
  ```rust
  i8, i16, i32, i64, isize
  u8, u16, u32, u64, usize
  f32, i32
  ```
- [x] add easy debug function
- [x] get mid point from vector
- [x] improve lib
- [x] add properties
- [x] improve magnitude calculation function
- [x] refactoring
- [ ] adding shape2d
- [ ] cleaning

Any suggestion please open issue or do a pull request

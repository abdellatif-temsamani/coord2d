# modules coord2d
[![Crate Status](https://img.shields.io/crates/v/coord2d?style=for-the-badge)](https://crates.io/crates/coord2d)
[![docs.rs](https://img.shields.io/docsrs/coord2d?style=for-the-badge)](https://docs.rs/coord2d/0.2.3/coord2d/)
![Crates.io](https://img.shields.io/crates/l/coord2d?style=for-the-badge)


rust lib for coordinate in 2d system

## example Coord

```rust
use coord2d::*;
fn main () {
    let a: coords::Coord = new_coord!(1, 2.0);
    let x: i32 = 3.0;
    let y: i32 = 3;
    let b: coords::Coord = new_coord!(x, y);

    // math
    let c: coords::Coord = a + b;
    let d: coords::Coord = a - c;
    let e: coords::Coord = a * d;
    let f: coords::Coord = a / a;

    // to vec
    let g: Vec<i32> = a.to_vec();

    // to tuple
    let i: (i32, i32) = c.to_tuple();
    // split
    let (r, o) = e.split();
    let mag: i32 = t.get_magnitude();
    let mid: coords::Coord = m.get_midpoint();

    println!("let a: coord::Coord = coord::new(1, 2); | {:?}", a);
    println!("\nlet (x , y) = (3, 3);\nlet b: coord::Coord = coord::new(x, y); | {:?}", b);
    println!("\nlet c: coord::Coord = a + b; | {:?}", c);
    println!("\nlet d: coord::Coord = a - c; | {:?}", d);
    println!("\nlet e: coord::Coord = a * d; | {:?}", e);
    println!("\nlet f: coord::Coord = a / a; | {:?}", f);
    println!("\n let g: Vec<i32> = a.to_vec(); {:?}", g);
    println!("\n let i: (i32, i32) = c.to_tuple(); | {:?}", i);
    println!("\n let (r, o) = e.split(); r={}, o={}" ,r,o);
    println!("\nlet mag = t.get_magnitude(); | {}", mag);
    println!("\nlet mid: coords::Coord = m.get_midpoint(); | {:?}" , mid);
}
```
## example Vectors

```rust
use coord2d::*;

fn main () {
    let a: coords::Coord = new_coord!(1, 2.0);
    let x: i32 = 3.0;
    let y: i32 = 3;
    let b: coords::Coord = new_coord!(x, y);

    let d: coords::Coord = a - b;
    let e: coords::Coord = a * d;
    let h:vectors::Vector = new_vector!(a, b);
    let l:vectors::Vector = new_vector!(d, e);
    let m = h + l;
    let s = h + l;
    let w = m + h;
    let t = h + s;
    let vec = s.to_vec();
    let tup = s.to_tuple();
    let mag = t.get_magnitude();

    println!("\nlet h = points::new(a, b); | {:?}", h);
    println!("\nlet m = h + l; | {:?}", m);
    println!("\nlet s = h + l; | {:?}", s);
    println!("\nlet w = m + h; | {:?}", w);
    println!("\nlet t = h + s; | {:?}", t);
    println!("\nlet mag = t.get_magnitude(); | {}", mag);
    println!("\nlet vec = s.to_vec(); | {:?}" , vec);
    println!("let tup = s.to_tuple(); | {:?}" , tup);
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

- [X] basic math
- [X] add fn give magnitude of vector
- [X] fix coord to accept other integers and floats types such as:
    ```rust
    i8, i16, i32, i64, isize
    u8, u16, u32, u64, usize
    f32, i32
    ```
- [X] add easy debug function
- [X] get mid point from vector
- [x] improve lib
- [x] add properties
- [x] improve magnitude calculation function

Any suggestion please open issue or do a pull request
## stats

![Anurag's GitHub stats](https://github-readme-stats.vercel.app/api?username=abdellatif-dev&show_icons=true&theme=radical)

[![Top Langs](https://github-readme-stats.vercel.app/api/top-langs/?username=abdellatif-dev&layout=compact&show_icons=true&theme=radical)](https://github.com/abdellatif-dev/github-readme-stats)

you contact us over contact@abdellatifdev.xyz

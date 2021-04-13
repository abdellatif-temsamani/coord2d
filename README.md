# modules coord2d
[![Crate Status](https://img.shields.io/crates/v/coord2d?style=for-the-badge)](https://crates.io/crates/coord2d)
[![docs.rs](https://img.shields.io/docsrs/coord2d?style=for-the-badge)](https://docs.rs/coord2d/0.1.9/coord2d/)
![Crates.io](https://img.shields.io/crates/l/coord2d?style=for-the-badge)


rust lib for coordinate in 2d system

## example Coord

```rust
use coord2d::*;
fn main () {
    let a: coords::Coord = coords::new(1.0, 2.0);
    let x: f64 = 3.0;
    let y: f64 = 3.0;
    let b: coords::Coord = coords::new(x, y);
    
    // math
    let c: coords::Coord = a + b;
    let d: coords::Coord = a - c;
    let e: coords::Coord = a * d;
    let f: coords::Coord = a / a;
    
    // to vec
    let g: Vec<f64> = a.to_vec();
    
    // to tuple
    let i: (f64, f64) = c.to_tuple();
    // split
    let (r, o) = e.split();
    
    println!("let a: coord::Coord = coord::new(1, 2); | {:?}", a);
    println!("\nlet (x , y) = (3, 3);\nlet b: coord::Coord = coord::new(x, y); | {:?}", b);
    println!("\nlet c: coord::Coord = a + b; | {:?}", c);
    println!("\nlet d: coord::Coord = a - c; | {:?}", d);
    println!("\nlet e: coord::Coord = a * d; | {:?}", e);
    println!("\nlet f: coord::Coord = a / a; | {:?}", f);
    println!("\n let g: Vec<f64> = a.to_vec(); {:?}", g);
    println!("\n let i: (f64, f64) = c.to_tuple(); | {:?}", i);
    println!("\n let (r, o) = e.split(); r={}, o={}" ,r,o);
}
```
## example Vectors

```rust
use coord2d::*;

fn main () {
    let a: coords::Coord = coords::new(1.0, 2.0);
    let x: f64 = 3.0;
    let y: f64 = 3.0;
    let b: coords::Coord = coords::new(x, y);
    
    let d: coords::Coord = a - b;
    let e: coords::Coord = a * d;
    let h = vectors::new(a, b);
    let l = vectors::new(a, e);
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

# TO DO

- [x] basic math
- [X] add fn give magnitude of vector
- [ ] add properties

## stats

![Anurag's GitHub stats](https://github-readme-stats.vercel.app/api?username=abdellatif-dev&show_icons=true&theme=radical)

[![Top Langs](https://github-readme-stats.vercel.app/api/top-langs/?username=abdellatif-dev&layout=compact&show_icons=true&theme=radical)](https://github.com/abdellatif-dev/github-readme-stats)

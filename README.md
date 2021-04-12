# modules coord2d

rust lib for coordinate in 2d system

## example 1

```rust
use coord2d::*;

fn main() {
    let coord1: coords::Coord = coords::new(3, 4);
    let coord2: coords::Coord = coords::new(5, 13);
    let sum: coords::Coord = coord1 + coord2;

    println!("{:?}", sum);
}
```
## example 2 

```rust
use coord2d::*;

fn main() {
    let coord1: coords::Coord = coords::new(3, 4);
    let coord2: coords::Coord = coords::new(5, 13);

    let h = vectors::new(coord1, coord2);
    let l = vectors::new(coord1, coord2);

    let m = h + l;
    let s = h - l;

    println!("{:?}", h);
    println!("{:?}", m);
    println!("{:?}", s);
    
}
```

# TO DO

- [x] basic math
- [X] add fn give magnitude of vector
- [ ] add properties

## stats

![Anurag's GitHub stats](https://github-readme-stats.vercel.app/api?username=abdellatif-dev&show_icons=true&theme=radical)

[![Top Langs](https://github-readme-stats.vercel.app/api/top-langs/?username=abdellatif-dev&layout=compact&show_icons=true&theme=radical)](https://github.com/abdellatif-dev/github-readme-stats)

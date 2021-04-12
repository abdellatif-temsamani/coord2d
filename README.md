#modules coord2d
rust lib for coordinate in 2d system

## example

```rust
use coord2d::*;

fn main() {
    let coord1: coords::Coord = coords::new(3, 4);
    let coord2: coords::Coord = coords::new(5, 13);
    let sum: coords::Coord = coord1 + coord2;

    println!("{:?}", sum);

    let line : vectors::Vector = vectors::new(coord1, coord2);

    println!("{:?}", line);
}

```

# TO DO

- [x] basic math
- [X] add fn give magnitude of vector
- [ ] add properties

## stats

![Anurag's GitHub stats](https://github-readme-stats.vercel.app/api?username=abdellatif-dev&show_icons=true&theme=radical)

[![Top Langs](https://github-readme-stats.vercel.app/api/top-langs/?username=abdellatif-dev&layout=compact&show_icons=true&theme=radical)](https://github.com/abdellatif-dev/github-readme-stats)

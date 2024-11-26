# **Coord2d**

[![Crate Status](https://img.shields.io/crates/v/coord2d?style=for-the-badge)](https://crates.io/crates/coord2d)
[![docs.rs](https://img.shields.io/docsrs/coord2d?style=for-the-badge)](https://docs.rs/coord2d/latest/coord2d/)
![License](https://img.shields.io/crates/l/coord2d?style=for-the-badge)

**A Rust library for working with 2D coordinates and vectors.**

## **Overview**

`Coord2D` provides a robust and flexible way to manage and manipulate 2D
coordinates and vectors in Rust. It supports various mathematical operations,
type conversions, and vector properties like magnitude and midpoint, making it
ideal for 2D geometry computations.

## **Features**

- **Coordinate Management:** Define and manipulate 2D coordinates with ease.
- **Mathematical Operations:** Perform addition, subtraction, multiplication,
  and division on coordinates.
- **Type Flexibility:** Supports multiple integer and floating-point types, such
  as `i32`, `f32`, `usize`, and more.
- **Vector Utilities:** Calculate magnitude, midpoint, and more.
- **Conversions:** Convert coordinates to vectors, tuples, or `Vec` formats.
- **Extensible:** Designed with modularity to support future enhancements like
  shapes.

## **Getting Started**

Add `coord2d` to your `Cargo.toml`:

```toml
[dependencies]
coord2d = "*"
```

## **Examples**

### **Working with Coordinates**

```rust
use coord2d::*;

fn main() {
    let a: coord::Coord = new_coord!(1, 2.0);
    let b: coord::Coord = new_coord!(3, 4.5);
}
```

### **Working with Vectors**

```rust
use coord2d::*;

fn main() {
    let a: coord::Coord = new_coord!(2.5, 3.3);
    let b: coord::Coord = new_coord!(1.0, 1.5);

    let vector: vector::Vector = new_vector!(a, b);

    let magnitude = vector.get_magnitude(); // Calculate magnitude
    let midpoint = vector.get_midpoint(); // Get midpoint

    println!("Magnitude: {}", magnitude);
    println!("Midpoint: {:?}", midpoint);
}
```

## **Planned Features**

- [ ] **2D Shapes:** Add support for shape creation and manipulation.
- [ ] **Code Cleanup:** Refactor and clean up the codebase.

## **Contributing**

We welcome contributions to improve `coord2d`. If you have ideas or spot issues:

1. Open an issue on [GitHub](https://github.com/abdellatif-temsamani/coord2d).
2. Submit a pull request with your changes.

## **License**

This project is licensed under the [GPL License](LICENSE).

**Enjoy using `coord2d` for your projects? Give us a star! 🌟**

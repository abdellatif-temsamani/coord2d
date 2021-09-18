#![allow(dead_code)]
use std::ops;

/// # Coord sturt
#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

/// ## implementations
impl Coord {
    /// ### converting to a Vec
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let ve: Vec<i32> = a.to_vec();
    /// println!("{:?}", ve);
    /// ```
    pub fn to_vec(self: Coord) -> Vec<i32> {
        vec![self.x, self.y]
    }

    /// ### converting to a tuple
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let tu: (i32, i32) = a.to_tuple();
    /// let (x, y) = a.to_tuple();
    /// println!("{}, {}", x , y);
    /// println!("{:?}", tu);
    /// ```
    pub fn to_tuple(self: Coord) -> (i32, i32) {
        (self.x, self.y)
    }
}

/// ## operations

/// ### addition

/// ##### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let sum:coords::Coord = a + b;
/// println!("{:?}", sum);
/// ```
impl ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self: Coord, rhs: Coord) -> Coord {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// ### subtraction
/// ##### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let sub: coords::Coord = a - b;
/// println!("{:?}", sub);
/// ```
impl ops::Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self: Coord, rhs: Coord) -> Coord {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// ### multiplication
/// ##### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let mul:coords::Coord = a * b;
/// println!("{:?}", mul);
/// ```
impl ops::Mul<Coord> for Coord {
    type Output = Coord;

    fn mul(self: Coord, rhs: Coord) -> Coord {
        Coord {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

/// ### division
/// ##### Example
///
/// ``` rust
/// use coord2d::*;
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(1.4, 3.3);
/// let div: coords::Coord = a / b;
/// ```
impl ops::Div<Coord> for Coord {
    type Output = Coord;

    fn div(self: Coord, rhs: Coord) -> Coord {
        Coord {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

#![allow(dead_code)]
use std::ops;

/// # Coord sturt
#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

/// ## creating a new Coord
/// #### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coord::Coord = new_coord!(1, 2.0);
/// println!("{:?}", a);
/// // returns Coord {x: 1.0, y: 2.0}
/// let a: coord::Coord = new_coord!(2.0);
/// println!("{:?}", a);
/// // returns Coord {x: 0.0, y: 2.0}
/// ```
/// * the var **a** contains to var x and y as position in a graph (O, I, J)
/// * returns i32
#[macro_export]
macro_rules! new_coord {
    () => {
        $crate::coord::Coord { x: 0.0, y: 0.0 }
    };

    ($y:expr) => {
        $crate::coord::Coord {
            x: 0.0,
            y: $y as f64,
        }
    };

    ($x:expr, $y:expr) => {
        $crate::coord::Coord {
            x: $x as f64,
            y: $y as f64,
        }
    };
}

/// ## implementations
impl Coord {
    /// ### converting to a Vec
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coord::Coord = new_coord!(1, 2.0);
    /// let ve: Vec<f64> = a.to_vec();
    /// println!("{:?}", ve);
    /// ```
    pub fn to_vec(self: Coord) -> Vec<f64> {
        vec![self.x, self.y]
    }

    /// ### converting to a tuple
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coord::Coord = new_coord!(1, 2.0);
    /// let tu: (f64, f64) = a.to_tuple();
    /// let (x, y) = a.to_tuple();
    /// println!("{}, {}", x , y);
    /// println!("{:?}", tu);
    /// ```
    pub fn to_tuple(self: Coord) -> (f64, f64) {
        (self.x, self.y)
    }
}

/// ## operations

/// ### addition

/// ##### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coord::Coord = new_coord!(1, 2.0);
/// let b: coord::Coord = new_coord!(0.4, 3.3);
/// let sum:coord::Coord = a + b;
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
/// let a: coord::Coord = new_coord!(1, 2.0);
/// let b: coord::Coord = new_coord!(0.4, 3.3);
/// let sub: coord::Coord = a - b;
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
/// let a: coord::Coord = new_coord!(1, 2.0);
/// let b: coord::Coord = new_coord!(0.4, 3.3);
/// let mul:coord::Coord = a * b;
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
/// let a: coord::Coord = new_coord!(1, 2.0);
/// let b: coord::Coord = new_coord!(1.4, 3.3);
/// let div: coord::Coord = a / b;
/// ```
impl ops::Div<Coord> for Coord {
    type Output = Self;

    fn div(self: Self, rhs: Self) -> Self {
        if rhs.x == 0.0 || rhs.y == 0.0 {
            panic!("[Coord]: division by 0");
        }

        Coord {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

/// ### PartialEq
/// ##### Example
///
/// ``` rust
/// use coord2d::*;
///
/// let a: coord::Coord = new_coord!(1.4, 3.3);
/// let b: coord::Coord = new_coord!(1.4, 3.3);
/// assert_eq!(a, b);
/// ```
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

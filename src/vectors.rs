#![allow(dead_code)]
use crate::coords;
use std::ops;
/// # Vector
/// it only Coord struct as input
#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub start: coords::Coord,
    pub end: coords::Coord,
    pub magnitude: f64,
}

/// ## creating a new Vector
/// #### Example
/// ```rust
/// use coord2d::{coords, vectors};
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let h:vectors::Vector = new_vector!(a, b);
/// println!("{:?}", h);
/// ```
/// * the var **h** contains to var start point and end point point
/// * while magnitude get calculated automatically
#[macro_export]
macro_rules! new_vector {
    () => {
        $crate::vectors::Vector {
            start: coords::Coord { x: 0, y: 0 },
            end: coords::Coord { x: 0, y: 0 },
            magnitude: 0.0,
        }
    };

    ($start:expr, $end:expr) => {
        vectors::Vector {
            start: $start,
            end: $end,
            magnitude: vectors::__magnitude__($start as coords::Coord, $end as coords::Coord),
        }
    };

    ($end:expr) => {
        vectors::Vector {
            start: coords::Coord { x: 0, y: 0 },
            end: $end,
            magnitude: vectors::__magnitude__(coords::Coord { x: 0, y: 0 }, $end as coords::Coord),
        }
    };
}
/// # magnitude
/// called when creating to a new vector
pub fn __magnitude__(start: coords::Coord, end: coords::Coord) -> f64 {
    let (x, y) = coords::Coord::to_tuple(end - start);

    let sum = (x.pow(2) + y.pow(2)) as f64;

    sum.sqrt()
}

impl Vector {
    /// ### converting to a Vec
    /// ##### Example
    /// ```rust
    /// use coord2d::{coords, vectors};
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let b: coords::Coord = new_coord!(0.4, 3.3);
    /// let s:vectors::Vector =new_vector!(a, b);
    /// let ve: Vec<coords::Coord> = s.to_vec();
    /// println!("{:?}", ve);
    /// }
    /// ```
    pub fn to_vec(self: Vector) -> Vec<coords::Coord> {
        vec![self.start, self.end]
    }

    /// ### converting to a tuple
    /// #### Example
    /// ```rust
    /// use coord2d::{coords, vectors};
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let b: coords::Coord = new_coord!(0.4, 3.3);
    /// let s:vectors::Vector = new_vector!(a, b);
    /// let tu:(coords::Coord, coords::Coord , f64) = s.to_tuple();
    /// println!("{:?}", tu);
    /// }
    /// ```
    pub fn to_tuple(self: Vector) -> (coords::Coord, coords::Coord, f64) {
        (self.start, self.end, self.magnitude)
    }

    /// ### spliting Vectors
    /// #### Example
    /// ```rust
    /// use coord2d::{coords, vectors};
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let b: coords::Coord = new_coord!(0.4, 3.3);
    /// let s:vectors::Vector =new_vector!(a, b);
    /// let (start, end , mag):(coords::Coord, coords::Coord , f64) = s.to_tuple();
    /// println!("{:?} {:?} {}", start, end , mag);
    /// ```
    pub fn split(self: Vector) -> (coords::Coord, coords::Coord, f64) {
        self.to_tuple()
    }

    /// ### spliting Vectors
    /// #### Example
    /// ```rust
    /// use coord2d::{coords, vectors};
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let b: coords::Coord = new_coord!(0.4, 3.3);
    /// let s:vectors::Vector =new_vector!(a, b);
    /// let mag = s.get_magnitude();
    /// println!("{}", mag);
    /// }
    /// ```
    pub fn get_magnitude(self: Vector) -> f64 {
        let mag_coord = self.end - self.start;
        let (x, y) = mag_coord.to_tuple();

        let sum = (x.pow(2) + y.pow(2)) as f64;

        sum.sqrt()
    }

    /// # geting mid point
    /// #### Example
    /// ```rust
    /// use coord2d::{coords, vectors};
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let b: coords::Coord = new_coord!(0.4, 3.3);
    /// let s: vectors::Vector =new_vector!(a, b);
    /// let mid: coords::Coord = s.get_midpoint();
    /// debug::debug(mid);
    /// }
    pub fn get_midpoint(self: Vector) -> coords::Coord {
        coords::Coord {
            x: self.start.x + self.end.x,
            y: self.start.y + self.end.y,
        }
    }
}

/// ## math
/// ### addition
/// ##### Example
/// ```rust
/// use coord2d::{coords, vectors};
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let s: vectors::Vector = new_vector!(a, b);
/// let h: vectors::Vector = new_vector!(b, a);
/// let m: vectors::Vector = h + s;
/// println!("{:?}", m);
/// ```
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self: Vector, rhs: Vector) -> Vector {
        let start = self.start + rhs.start;
        let end = self.end + rhs.end;

        Vector {
            start,
            end,
            magnitude: __magnitude__(start, end),
        }
    }
}

/// ### subtraction
/// ##### Example
/// ```rust
/// use coord2d::{coords, vectors};
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let s: vectors::Vector = new_vector!(a, b);
/// let h: vectors::Vector = new_vector!(b, a);
/// let m: vectors::Vector = h - s;
/// println!("{:?}", m);
/// ```
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self: Vector, rhs: Vector) -> Vector {
        let start = self.start - rhs.start;
        let end = self.end - rhs.end;

        Vector {
            start,
            end,
            magnitude: __magnitude__(start, end),
        }
    }
}

/// ### multiplication
/// ##### Example
/// ```rust
/// use coord2d::{coords, vectors};
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let s: vectors::Vector = new_vector!(a, b);
/// let h: vectors::Vector = new_vector!(b, a);
/// let m: vectors::Vector = h * s;
/// println!("{:?}", m);
/// ```
impl ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self: Vector, rhs: Vector) -> Vector {
        let start = self.start * rhs.start;
        let end = self.end * rhs.end;

        Vector {
            start,
            end,
            magnitude: __magnitude__(start, end),
        }
    }
}

/// ### division
/// ##### Example
/// ```rust
/// use coord2d::{coords, vectors};
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let s: vectors::Vector = new_vector!(a, b);
/// let h: vectors::Vector = new_vector!(b, a);
/// let m: vectors::Vector = h / s;
/// println!("{:?}", m);
/// ```
impl ops::Div<Vector> for Vector {
    type Output = Vector;

    fn div(self: Vector, rhs: Vector) -> Vector {
        let start = self.start / rhs.start;
        let end = self.end / rhs.end;

        Vector {
            start,
            end,
            magnitude: __magnitude__(start, end),
        }
    }
}

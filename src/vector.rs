#![allow(dead_code)]
use crate::coord;
use std::ops;

/// # Vector
/// it only Coord struct as input
#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub start: coord::Coord,
    pub end: coord::Coord,
    pub magnitude: f64,
}

/// ## creating a new Vector
/// #### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coord::Coord = new_coord!(1, 2.0);
/// let b: coord::Coord = new_coord!(0.4, 3.3);
/// let h:vector::Vector = new_vector!(a, b);
/// ```
/// * the var **h** contains to var start point and end point point
/// * while magnitude get calculated automatically
#[macro_export]
macro_rules! new_vector {
    () => {
        $crate::vector::Vector {
            start: coord::Coord { x: 0.0, y: 0.0 },
            end: coord::Coord { x: 0.0, y: 0.0 },
            magnitude: 0.0,
        }
    };

    ($end:expr) => {
        vector::Vector {
            start: coord::Coord { x: 0.0, y: 0.0 },
            end: $end,
            magnitude: vector::get_magnitude(coord::Coord { x: 0.0, y: 0.0 }, $end as coord::Coord),
        }
    };

    ($start:expr, $end:expr) => {
        vector::Vector {
            start: $start,
            end: $end,
            magnitude: vector::get_magnitude($start as coord::Coord, $end as coord::Coord),
        }
    };
}

/// # magnitude
/// called when creating to a new vector
pub fn get_magnitude(start: coord::Coord, end: coord::Coord) -> f64 {
    (((end.x - start.x).powi(2) + (end.y - start.y).powi(2)) as f64).sqrt()
}

/// ## implementations
impl Vector {
    /// ### converting to a Vec
    /// ##### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coord::Coord = new_coord!(1, 2.0);
    /// let b: coord::Coord = new_coord!(0.4, 3.3);
    /// let s:vector::Vector =new_vector!(a, b);
    /// let ve: Vec<coord::Coord> = s.to_vec();
    /// println!("{:?}", ve);
    /// ```
    pub fn to_vec(self: Vector) -> Vec<coord::Coord> {
        vec![self.start, self.end]
    }

    /// ### converting to a tuple
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coord::Coord = new_coord!(1, 2.0);
    /// let b: coord::Coord = new_coord!(0.4, 3.3);
    /// let s:vector::Vector = new_vector!(a, b);
    /// let tu:(coord::Coord, coord::Coord , f64) = s.to_tuple();
    /// println!("{:?}", tu);
    /// ```
    pub fn to_tuple(self: Vector) -> (coord::Coord, coord::Coord, f64) {
        (self.start, self.end, self.magnitude)
    }

    /// ### spliting Vectors
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coord::Coord = new_coord!(1, 2.0);
    /// let b: coord::Coord = new_coord!(0.4, 3.3);
    /// let s:vector::Vector =new_vector!(a, b);
    /// let mag = s.get_magnitude();
    /// println!("{}", mag);
    /// ```
    pub fn get_magnitude(self: Vector) -> f64 {
        (((self.end.x - self.start.x).powi(2) + (self.end.y - self.start.y).powi(2)) as f64).sqrt()
    }

    /// # geting mid point
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coord::Coord = new_coord!(1, 2.0);
    /// let b: coord::Coord = new_coord!(0.4, 3.3);
    /// let s: vector::Vector = new_vector!(a, b);
    /// let mid: coord::Coord = s.get_midpoint();
    pub fn get_midpoint(self: Vector) -> coord::Coord {
        coord::Coord {
            x: (self.start.x + self.end.x) / 2.0,
            y: (self.start.y + self.end.y) / 2.0,
        }
    }
}

/// ## math
/// ### addition
/// ##### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coord::Coord = new_coord!(1, 2.0);
/// let b: coord::Coord = new_coord!(0.4, 3.3);
/// let m: vector::Vector = new_vector!(a, b) + new_vector!(b, a) ;
/// println!("{:?}", m);
/// ```
impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self: Vector, rhs: Vector) -> Vector {
        Vector {
            start: self.start + rhs.start,
            end: self.end + rhs.end,
            magnitude: get_magnitude(self.start, self.end),
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
/// let m: vector::Vector = new_vector!(a, b) - new_vector!(b, a) ;
/// println!("{:?}", m);
/// ```
impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self: Vector, rhs: Vector) -> Vector {
        Vector {
            start: self.start - rhs.start,
            end: self.end - rhs.end,
            magnitude: get_magnitude(self.start, self.end),
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
/// let m: vector::Vector = new_vector!(a, b) * new_vector!(b, a) ;
/// println!("{:?}", m);
/// ```
impl ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self: Vector, rhs: Vector) -> Vector {
        Vector {
            start: self.start * rhs.start,
            end: self.end * rhs.end,
            magnitude: get_magnitude(self.start, self.end),
        }
    }
}

/// ### division
/// ##### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coord::Coord = new_coord!(1, 2.0);
/// let b: coord::Coord = new_coord!(4.03, 3.3);
/// let m: vector::Vector = new_vector!(a, b) / new_vector!(b, a);
/// ```
impl ops::Div<Vector> for Vector {
    type Output = Self;

    fn div(self: Self, rhs: Self) -> Self {
        if rhs.end.x == 0.0 || rhs.end.y == 0.0 {
            panic!("[Vector]: division by 0");
        }

        Vector {
            start: self.start / rhs.start,
            end: self.end / rhs.end,
            magnitude: get_magnitude(self.start, self.end),
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
/// let v1: vector::Vector = new_vector!(a, b);
/// ```
impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

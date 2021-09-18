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


/// # magnitude
/// called when creating to a new vector
pub fn get_magnitude(start: coords::Coord, end: coords::Coord) -> f64 {
    (((end.x - start.x).pow(2) + (end.y - start.y).pow(2)) as f64).sqrt()
}

/// ## implementations
impl Vector {

    /// ### converting to a Vec
    /// ##### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let b: coords::Coord = new_coord!(0.4, 3.3);
    /// let s:vectors::Vector =new_vector!(a, b);
    /// let ve: Vec<coords::Coord> = s.to_vec();
    /// println!("{:?}", ve);
    /// ```
    pub fn to_vec(self: Vector) -> Vec<coords::Coord> {
        vec![self.start, self.end]
    }

    /// ### converting to a tuple
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let b: coords::Coord = new_coord!(0.4, 3.3);
    /// let s:vectors::Vector = new_vector!(a, b);
    /// let tu:(coords::Coord, coords::Coord , f64) = s.to_tuple();
    /// println!("{:?}", tu);
    /// ```
    pub fn to_tuple(self: Vector) -> (coords::Coord, coords::Coord, f64) {
        (self.start, self.end, self.magnitude)
    }

    /// ### spliting Vectors
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let b: coords::Coord = new_coord!(0.4, 3.3);
    /// let s:vectors::Vector =new_vector!(a, b);
    /// let mag = s.get_magnitude();
    /// println!("{}", mag);
    /// ```
    pub fn get_magnitude(self: Vector) -> f64 {
        (((self.end.x - self.start.x).pow(2) + (self.end.y - self.start.y).pow(2)) as f64).sqrt()
    }

    /// # geting mid point
    /// #### Example
    /// ```rust
    /// use coord2d::*;
    ///
    /// let a: coords::Coord = new_coord!(1, 2.0);
    /// let b: coords::Coord = new_coord!(0.4, 3.3);
    /// let s: vectors::Vector = new_vector!(a, b);
    /// let mid: coords::Coord = s.get_midpoint();
    pub fn get_midpoint(self: Vector) -> coords::Coord {
        coords::Coord {
            x: (self.start.x + self.end.x) / 2,
            y: (self.start.y + self.end.y) / 2,
        }
    }
}

/// ## math
/// ### addition
/// ##### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let m: vectors::Vector = new_vector!(a, b) + new_vector!(b, a) ;
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
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let m: vectors::Vector = new_vector!(a, b) - new_vector!(b, a) ;
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
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let m: vectors::Vector = new_vector!(a, b) * new_vector!(b, a) ;
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
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(4.03, 3.3);
/// let m: vectors::Vector = new_vector!(a, b) / new_vector!(b, a);
/// ```
impl ops::Div<Vector> for Vector {
    type Output = Vector;

    fn div(self: Vector, rhs: Vector) -> Vector {
        Vector {
            start: self.start / rhs.start,
            end: self.end / rhs.end,
            magnitude: get_magnitude(self.start, self.end),
        }
    }
}

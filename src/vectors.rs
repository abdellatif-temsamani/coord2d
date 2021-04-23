use crate::coords;
use std::ops;
/// # Vector
/// it only Coord struct as input
#[derive(Clone, Copy , Debug)]
pub struct Vector {
    start: coords::Coord,
    end: coords::Coord,
    magnitude: f64,
}

/// ## creating a new Coord
/// #### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let b: coords::Coord = new_coord!(0.4, 3.3);
///     let h:vectors::Vector = new_vector!(a, b);
///     println!("{:?}", h);
/// }
/// ```
/// * the var **h** contains to var start point and end point point
/// * while magnitude get calculated automatically
pub fn new(start: coords::Coord, end: coords::Coord) -> Vector {
    
    let magnitude = magnitude(start, end);

    return Vector{start, end, magnitude};
}
#[macro_export]
macro_rules! new_vector {
    () => { $crate::vectors::new($crate::coords::Coord{x: 0.0 , y: 0.0}, $crate::coords::Coord{x: 0.0 , y: 0.0}) };
    ($start:expr, $end:expr) => { $crate::vectors::new($start as coords::Coord , $end as coords::Coord) };
    ($end:expr) => { $crate::vectors::new($crate::coords::Coord{x: 0.0 , y: 0.0}, $end as coords::Coord) };
}


/// - magnitude
/// called when creating to a new vector
fn magnitude(start: coords::Coord , end: coords::Coord) -> f64 {
    let (mut x,mut y) = coords::Coord::to_tuple(end - start);
    x = x * x;
    y = y * y;

    let sum = x + y;
    let magnitude = sum.sqrt();
    return magnitude;
}


impl Vector {
    /// ### converting to a Vec
    /// ##### Example  
    /// ```rust
    /// use coord2d::*;
    /// fn main () {
    ///     let a: coords::Coord = new_coord!(1, 2.0);
    ///     let b: coords::Coord = new_coord!(0.4, 3.3);
    ///     let s:vectors::Vector =new_vector!(a, b);
    ///     let ve: Vec<coords::Coord> = s.to_vec();
    ///     println!("{:?}", ve);
    /// }
    /// ```
    pub fn to_vec(self : Vector) -> Vec<coords::Coord> {
        return vec![self.start, self.end];
    }
    /// ### converting to a tuple
    /// #### Example  
    /// ```rust
    /// use coord2d::*;
    /// fn main () {
    ///     let a: coords::Coord = new_coord!(1, 2.0);
    ///     let b: coords::Coord = new_coord!(0.4, 3.3);
    ///     let s:vectors::Vector = new_vector!(a, b);
    ///     let tu:(coords::Coord, coords::Coord , f64) = s.to_tuple();
    ///     println!("{:?}", tu);
    /// }
    /// ```
    pub fn to_tuple(self : Vector) -> (coords::Coord, coords::Coord, f64) {
        return (self.start, self.end, self.magnitude);
        
    }
    /// ### spliting Vectors
    /// #### Example  
    /// ```rust
    /// use coord2d::*;
    /// fn main () {
    ///     let a: coords::Coord = new_coord!(1, 2.0);
    ///     let b: coords::Coord = new_coord!(0.4, 3.3);
    ///     let s:vectors::Vector =new_vector!(a, b);
    ///     let (start, end , mag):(coords::Coord, coords::Coord , f64) = s.to_tuple();
    ///     println!("{:?} {:?} {}", start, end , mag);
    /// }
    /// ```
    pub fn split(self: Vector) -> (coords::Coord, coords::Coord, f64) {
        return self.to_tuple();
    }
    /// ### spliting Vectors
    /// #### Example  
    /// ```rust
    /// use coord2d::*;
    /// fn main () {
    ///     let a: coords::Coord = new_coord!(1, 2.0);
    ///     let b: coords::Coord = new_coord!(0.4, 3.3);
    ///     let s:vectors::Vector =new_vector!(a, b);
    ///     let mag = s.get_magnitude();
    ///     println!("{}", mag);
    /// }
    /// ```
    pub fn get_magnitude(self : Vector) -> f64 {
        let mag_coord= self.end - self.start;
        let (mut x,mut y) = mag_coord.to_tuple();
        x = x * x;
        y = y * y;
    
        let sum = x + y;
        let magnitude = sum.sqrt();
        return magnitude;
    
    }
}

/// ## math
/// ### addition
/// ##### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let b: coords::Coord = new_coord!(0.4, 3.3);
///     let s: vectors::Vector = new_vector!(a, b);
///     let h: vectors::Vector = new_vector!(b, a);
///     let m: vectors::Vector = h + s;
///     println!("{:?}", m);
/// }
/// ```
impl ops::Add<Vector> for Vector{
    type Output = Vector;
    
    fn add(self: Vector, rhs: Vector) -> Vector {
        let start = self.start + rhs.start;
        let end = self.end + rhs.end;
        let magnitude = magnitude(start, end);

        return Vector{start, end, magnitude};
    }
}

/// ### subtraction
/// ##### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let b: coords::Coord = new_coord!(0.4, 3.3);
///     let s: vectors::Vector = new_vector!(a, b);
///     let h: vectors::Vector = new_vector!(b, a);
///     let m: vectors::Vector = h - s;
///     println!("{:?}", m);
/// }
/// ```
impl ops::Sub<Vector> for Vector{
    type Output = Vector;
    
    fn sub(self: Vector, rhs: Vector) -> Vector {
        let start = self.start - rhs.start;
        let end = self.end - rhs.end;
        let magnitude = magnitude(start, end);

        return Vector{start, end, magnitude};

    }
}

/// ### multiplication
/// ##### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let b: coords::Coord = new_coord!(0.4, 3.3);
///     let s: vectors::Vector = new_vector!(a, b);
///     let h: vectors::Vector = new_vector!(b, a);
///     let m: vectors::Vector = h * s;
///     println!("{:?}", m);
/// }
/// ```
impl ops::Mul<Vector> for Vector{
    type Output = Vector;
    
    fn mul(self: Vector, rhs: Vector) -> Vector {
        let start = self.start * rhs.start;
        let end = self.end * rhs.end;
        let magnitude = magnitude(start, end);

        return Vector{start, end, magnitude};

    }
}

/// ### division
/// ##### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let b: coords::Coord = new_coord!(0.4, 3.3);
///     let s: vectors::Vector = new_vector!(a, b);
///     let h: vectors::Vector = new_vector!(b, a);
///     let m: vectors::Vector = h / s;
///     println!("{:?}", m);
/// }
/// ```
impl ops::Div<Vector> for Vector{
    type Output = Vector;
    
    fn div(self: Vector, rhs: Vector) -> Vector {
        let start = self.start / rhs.start;
        let end = self.end / rhs.end;
        let magnitude = magnitude(start, end);

        return Vector{start, end, magnitude};
    }
}

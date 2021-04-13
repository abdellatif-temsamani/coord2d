#[allow(dead_code)]
use std::ops;

/// # Coord
/// for now it only take f64 as input

#[derive(Clone, Copy, Debug)]
pub struct Coord{
    pub x: f64,
    pub y: f64,
}

/// ## creating a new Coord
/// #### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = coords::new(1.0, 2.0);
///     println!("{:?}", a);
/// }
/// ```
/// * the var **a** contains to var x and y as position in a graph (O, I, J)

pub fn new(x: f64, y: f64) -> Coord {
    return Coord{x, y};
}

/// ## implementations
impl  Coord {
    
    /// ### converting to a Vec
    /// #### Example  
    /// ```rust
    /// use coord2d::*;
    /// fn main () {
    ///     let a: coords::Coord = coords::new(1.0, 2.0);
    ///     let ve: Vec<f64> = a.to_vec();
    ///     println!("{:?}", ve);
    /// }
    /// ```
    pub fn to_vec(self: Coord ) -> Vec<f64> {
        return vec![self.x, self.y];
    }
    

    /// ### converting to a tuple
    /// #### Example  
    /// ```rust
    /// use coord2d::*;
    /// fn main () {
    ///     let a: coords::Coord = coords::new(1.0, 2.0);
    ///     let tu: (f64, f64) = a.to_tuple();
    ///     println!("{:?}", tu);
    /// }
    /// ```
    pub fn to_tuple(self :Coord) -> (f64, f64) {
        return (self.x, self.y);
        
    }

    /// ### spliting Coord
    /// #### Example  
    /// ```rust
    /// use coord2d::*;
    /// fn main () {
    ///     let a: coords::Coord = coords::new(1.0, 2.0);
    ///     let (x, y) = a.split();
    ///     println!("{}, {}", x , y);
    /// }
    /// ```
    pub fn split(self: Coord) -> (f64, f64) {
        return self.to_tuple();
    }
}

/// ## math
/// ### addition
/// ##### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = coords::new(1.0, 2.0);
///     let b: coords::Coord = coords::new(5.4, 3.3);
///     let sum:coords::Coord = a + b;
///     println!("{:?}", sum);
/// }
/// ```
impl ops::Add<Coord> for Coord{
    type Output = Coord;
    
    fn add(self: Coord, rhs: Coord) -> Coord {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Coord{x, y};
    }
}
/// ### subtraction
/// ##### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = coords::new(1.0, 2.0);
///     let b: coords::Coord = coords::new(5.4, 3.3);
///     let sub:coords::Coord = a - b;
///     println!("{:?}", sub);
/// }
/// ```

impl ops::Sub<Coord> for Coord{
    type Output = Coord;
    
    fn sub(self: Coord, rhs: Coord) -> Coord {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        return Coord{x, y};
    }
}

/// ### multiplication
/// ##### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = coords::new(1.0, 2.0);
///     let b: coords::Coord = coords::new(5.4, 3.3);
///     let mul:coords::Coord = a * b;
///     println!("{:?}", mul);
/// }
/// ```
impl ops::Mul<Coord> for Coord{
    type Output = Coord;
    
    fn mul(self: Coord, rhs: Coord) -> Coord {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;

        return Coord{x, y};
    }
}

/// ### division
/// ##### Example  
/// ```rust
/// use coord2d::*;
/// fn main () {
///     let a: coords::Coord = coords::new(1.0, 2.0);
///     let b: coords::Coord = coords::new(5.4, 3.3);
///     let div:coords::Coord = a / b;
///     println!("{:?}", div);
/// }
/// ```
impl ops::Div<Coord> for Coord{
    type Output = Coord;
    
    fn div(self: Coord, rhs: Coord) -> Coord {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;

        return Coord{x, y};
    }
}


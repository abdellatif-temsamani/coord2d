#[allow(dead_code)]
use std::ops;

/// # Coord
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
///     let a: coords::Coord = new_coord!(1, 2.0);
///     println!("{:?}", a);
///     // returns Coord {x: 1.0, y: 2.0}
///     let a: coords::Coord = new_coord!(2.0);
///     println!("{:?}", a);
///     // returns Coord {x: 0.0, y: 2.0}
/// }
/// ```
/// * the var **a** contains to var x and y as position in a graph (O, I, J)
/// * it convert i32 ,i64, u32 ,u64 , usize ,isize and f32 to f64
#[macro_export]
macro_rules! new_coord {
    () => { $crate::coords::Coord {x: 0.0, y: 0.0} };
    ($x:expr, $y:expr) => {
        
        $crate::coords::Coord { x: $x as f64, y: $y as f64}
    };
    ($y:expr) => {
        
        $crate::coords::Coord { x: 0.0 , y: $y as f64}
    };
}

/// ## implementations
impl  Coord {
    
    /// ### converting to a Vec
    /// #### Example  
    /// ```rust
    /// use coord2d::*;
    /// fn main () {
    ///     let a: coords::Coord = new_coord!(1, 2.0);
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
    ///     let a: coords::Coord = new_coord!(1, 2.0);
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
    ///     let a: coords::Coord = new_coord!(1, 2.0);
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
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let b: coords::Coord = new_coord!(0.4, 3.3);
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
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let b: coords::Coord = new_coord!(0.4, 3.3);
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
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let b: coords::Coord = new_coord!(0.4, 3.3);
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
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let b: coords::Coord = new_coord!(0.4, 3.3);
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


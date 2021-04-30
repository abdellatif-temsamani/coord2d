use std::fmt::Debug;
/// # debug
/// ## example 
/// ```rust
/// use coord2d::*;
/// 
/// fn main () {
///     let a: coords::Coord = new_coord!(1, 2.0);
///     let x: f64 = 3.0;
///     let y: i32 = 3;
///     let b: coords::Coord = new_coord!(x, y);
///      
///     let h:vectors::Vector = new_vector!(a, b);
///     
///     debug::debug(a);
///     debug::debug(h);
/// }
/// ```
/// - easy to print vector or coord object
pub fn debug <T: Debug>(db : T) {
    println!("{:?}", db);
}

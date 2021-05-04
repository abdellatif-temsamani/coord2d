use std::{any::type_name, fmt::Debug};

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

/// # get type of var
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
///     let type_a = debug::type_of(a);
///     let type_h = debug::type_of(h);
///
///     println!("{}", type_a);
///     println!("{}", type_h);
/// }
/// ```
/// - easy to print vector or coord object
pub fn type_of<T> (_t :T) -> &'static str {
    type_name::<T>()
}

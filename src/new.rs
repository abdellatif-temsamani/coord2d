/// ## creating a new Coord
/// #### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// println!("{:?}", a);
/// // returns Coord {x: 1.0, y: 2.0}
/// let a: coords::Coord = new_coord!(2.0);
/// println!("{:?}", a);
/// // returns Coord {x: 0.0, y: 2.0}
/// ```
/// * the var **a** contains to var x and y as position in a graph (O, I, J)
/// * returns i32
#[macro_export]
macro_rules! new_coord {
    () => {
        $crate::coords::Coord { x: 0, y: 0 }
    };

    ($y:expr) => {
        $crate::coords::Coord { x: 0, y: $y as i32 }
    };

    ($x:expr, $y:expr) => {
        $crate::coords::Coord {
            x: $x as i32,
            y: $y as i32,
        }
    };
}

/// ## creating a new Vector
/// #### Example
/// ```rust
/// use coord2d::*;
///
/// let a: coords::Coord = new_coord!(1, 2.0);
/// let b: coords::Coord = new_coord!(0.4, 3.3);
/// let h:vectors::Vector = new_vector!(a, b);
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

    ($end:expr) => {
        vectors::Vector {
            start: coords::Coord { x: 0, y: 0 },
            end: $end,
            magnitude: vectors::get_magnitude(coords::Coord { x: 0, y: 0 }, $end as coords::Coord),
        }
    };

    ($start:expr, $end:expr) => {
        vectors::Vector {
            start: $start,
            end: $end,
            magnitude: vectors::get_magnitude($start as coords::Coord, $end as coords::Coord),
        }
    };
}

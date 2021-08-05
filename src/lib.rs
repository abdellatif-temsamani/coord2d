#[allow(dead_code)]
#[macro_use]
pub mod coords;

#[macro_use]
pub mod vectors;

#[macro_use]
pub mod debug;

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_coords() {
        let a: coords::Coord = new_coord!(1, 2.0);
        let o: coords::Coord = new_coord!(2.0);
        let x: i32 = 3;
        let y: f64 = 3.0;
        let b: coords::Coord = new_coord!(x, y);

        // math
        let c: coords::Coord = a + b;
        let d: coords::Coord = a - c;
        let e: coords::Coord = a * d;
        let f: coords::Coord = o / a;

        // to vec
        let g: Vec<i32> = a.to_vec();

        // to tuple
        let i: (i32, i32) = c.to_tuple();

        // split
        let (r, o) = e.to_tuple();

        println!("let a: coord::Coord = new_coord!(1, 2.0); | {:?}", a);
        println!("let o: coord::Coord = new_coord!(2.0); | {:?}", o);
        println!(
            "\nlet (x , y) = (3.0, 3);\nlet b: coords::Coord = new_coord!(x, y); | {:?}",
            b
        );
        println!("\nlet c: coord::Coord = a + b; | {:?}", c);
        println!("\nlet d: coord::Coord = a - c; | {:?}", d);
        println!("\nlet e: coord::Coord = a * d; | {:?}", e);
        println!("\nlet f: coord::Coord = a / a; | {:?}", f);
        println!("\n let g: Vec<i32> = a.to_vec(); {:?}", g);
        println!("\n let i: (i32, i32) = c.to_tuple(); | {:?}", i);
        println!("\n let (r, o) = e.to_tuple(); r={}, o={}", r, o);
    }

    #[test]
    fn test_vector() {
        let a: coords::Coord = new_coord!(2.5, 3.3);
        let o: coords::Coord = new_coord!(2.5);
        let x: i32 = 3;
        let y: f32 = 3.0;
        let b: coords::Coord = new_coord!(x, y);

        let d: coords::Coord = a - b;
        let e: coords::Coord = a * d;

        let h: vectors::Vector = new_vector!();
        let f: vectors::Vector = new_vector!(o);
        let l: vectors::Vector = new_vector!(a, e);

        let m: vectors::Vector = h + l;
        let s: vectors::Vector = h + l;
        let w: vectors::Vector = m + h;
        let t: vectors::Vector = h + s;

        let vec: Vec<coords::Coord> = s.to_vec();
        let tup: (coords::Coord, coords::Coord, f64) = s.to_tuple();
        let mag: f64 = t.get_magnitude();
        let mid: coords::Coord = m.get_midpoint();
        println!("\nlet h:vectors::Vector = new_vector!(); | {:?}", h);
        println!("\nlet f:vectors::Vector = new_vector!(); | {:?}", f);
        println!("\nlet l:vectors::Vector = new_vector!(a, e); | {:?}", l);
        println!("\nlet m = h + l; | {:?}", m);
        println!("\nlet s = h + l; | {:?}", s);
        println!("\nlet w = m + h; | {:?}", w);
        println!("\nlet t = h + s; | {:?}", t);
        println!("\nlet mag = t.get_magnitude(); | {}", mag);
        println!("\nlet vec = s.to_vec(); | {:?}", vec);
        println!("\nlet tup = s.to_tuple(); | {:?}", tup);
        println!("\nlet mid: coords::Coord = m.get_midpoint(); | {:?}", mid);
    }
    #[test]
    fn test_debug() {
        let a: coords::Coord = new_coord!(2.5, 3.3);
        let o: coords::Coord = new_coord!(2.5);
        let l: vectors::Vector = new_vector!(a, o);

        debug::debug(a);
        debug::debug(l);

        let _type = debug::type_of(a);
        println!("{}", _type);
    }
}

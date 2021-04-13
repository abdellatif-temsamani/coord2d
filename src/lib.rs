#[allow(dead_code)]
pub mod coords;
pub mod vectors;
#[cfg(test)]
mod test {
    use crate::coords;
    use crate::vectors;
    
    #[test]
    fn test_coords () {

        let a: coords::Coord = coords::new(3.0, 2.0);
        let x: f64 = 3.0;
        let y: f64 = 3.0;
        let b: coords::Coord = coords::new(x, y);
        
        // math
        let c: coords::Coord = a + b;
        let d: coords::Coord = a - c;
        let e: coords::Coord = a * d;
        let f: coords::Coord = a / a;
        
        // to vec
        let g: Vec<f64> = a.to_vec();
        
        // to tuple
        let i: (f64, f64) = c.to_tuple();

    
        // split
        let (r, o) = e.split();
        
        println!("let a: coord::Coord = coord::new(1, 2); | {:?}", a);
        println!("\nlet (x , y) = (3, 3);\nlet b: coord::Coord = coord::new(x, y); | {:?}", b);
        println!("\nlet c: coord::Coord = a + b; | {:?}", c);
        println!("\nlet d: coord::Coord = a - c; | {:?}", d);
        println!("\nlet e: coord::Coord = a * d; | {:?}", e);
        println!("\nlet f: coord::Coord = a / a; | {:?}", f);
        println!("\n let g: Vec<f64> = a.to_vec(); {:?}", g);
        println!("\n let i: (f64, f64) = c.to_tuple(); | {:?}", i);
        println!("\n let (r, o) = e.split(); r={}, o={}" ,r,o);
    }

    #[test]
    fn test_vector () {


        let a: coords::Coord = coords::new(1.0, 2.0);
        let x: f64 = 3.0;
        let y: f64 = 3.0;
        let b: coords::Coord = coords::new(x, y);
        
        let d: coords::Coord = a - b;
        let e: coords::Coord = a * d;

        let h:vectors::Vector = vectors::new(a, b);
        let l:vectors::Vector = vectors::new(a, e);

        let m:vectors::Vector = h + l;
        let s:vectors::Vector = h + l;
        let w:vectors::Vector = m + h;
        let t:vectors::Vector = h + s;

        let vec: Vec<coords::Coord> = s.to_vec();
        let tup:(coords::Coord, coords::Coord , f64) = s.to_tuple();
        let mag = t.get_magnitude();
        
        println!("\nlet h = points::new(a, b); | {:?}", h);
        println!("\nlet m = h + l; | {:?}", m);
        println!("\nlet s = h + l; | {:?}", s);
        println!("\nlet w = m + h; | {:?}", w);
        println!("\nlet t = h + s; | {:?}", t);
        println!("\nlet mag = t.get_magnitude(); | {}", mag);
        println!("\nlet vec = s.to_vec(); | {:?}" , vec);
        println!("let tup = s.to_tuple(); | {:?}" , tup);

    }
}

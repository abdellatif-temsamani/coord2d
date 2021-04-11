#[allow(dead_code)]
mod coord;


#[test]
#[cfg(test)]
fn test_coords () {
    use coord::Coord;

    let a: Coord = coord::new(1, 2);
    let (x , y) = (3, 3);
    let b: Coord = coord::new(x, y);
    
    // math
    let c: Coord = a + b;
    let d: Coord = a - c;
    let e: Coord = a * d;
    let f: Coord = a / a;
    
    // to vec
    let g: Vec<i32> = coord::Coord::to_vec(a);
    
    println!("let a: coord::Coord = coord::new(1, 2); | {:?}", a);
    println!("\nlet (x , y) = (3, 3);\nlet b: coord::Coord = coord::new(x, y); | {:?}", b);
    println!("\nlet c: coord::Coord = a + b; | {:?}", c);
    println!("\nlet d: coord::Coord = a - c; | {:?}", d);
    println!("\nlet e: coord::Coord = a * d; | {:?}", e);
    println!("\nlet f: coord::Coord = a / a; | {:?}", f);
    println!("\nlet g: Vec<i32> = coord::Coord::to_vec(a); | {:?}", g);

}
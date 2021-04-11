#[allow(dead_code)]
use std::ops;

#[derive(Clone, Copy , Debug)]
pub struct Coord{
    pub x: i32,
    pub y: i32,
}
pub fn new(x: i32, y: i32) -> Coord {
    return Coord{x, y};
}

// math
impl ops::Add<Coord> for Coord{
    type Output = Coord;
    
    fn add(self: Coord, rhs: Coord) -> Coord {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        return Coord{x, y};
    }
}

impl ops::Sub<Coord> for Coord{
    type Output = Coord;
    
    fn sub(self: Coord, rhs: Coord) -> Coord {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        return Coord{x, y};
    }
}

impl ops::Mul<Coord> for Coord{
    type Output = Coord;
    
    fn mul(self: Coord, rhs: Coord) -> Coord {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;

        return Coord{x, y};
    }
}

impl ops::Div<Coord> for Coord{
    type Output = Coord;
    
    fn div(self: Coord, rhs: Coord) -> Coord {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;

        return Coord{x, y};
    }
}

impl Coord {
    pub fn to_vec(coord :Coord) -> Vec<i32> {
        return vec![coord.x, coord.y];
    }
    
    pub fn to_tuple(coord :Coord) -> (i32, i32) {
        return (coord.x, coord.y);
        
    }
}

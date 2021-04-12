#[allow(dead_code)]
use std::ops;

#[derive(Clone, Copy , Debug)]
pub struct Coord{
    pub x: f64,
    pub y: f64,
}
pub fn new(x: f64, y: f64) -> Coord {
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
    pub fn to_vec(coord :Coord) -> Vec<f64> {
        return vec![coord.x, coord.y];
    }
    
    pub fn to_tuple(coord :Coord) -> (f64, f64) {
        return (coord.x, coord.y);
        
    }

    pub fn split(coord: Coord) -> (f64, f64) {
        return self::Coord::to_tuple(coord);
    }

}

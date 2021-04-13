use crate::coords;
use std::ops;

#[derive(Clone, Copy , Debug)]
pub struct Vector {
    start: coords::Coord,
    end: coords::Coord,
    magnitude: f64,
}

pub fn new(start: coords::Coord, end: coords::Coord) -> Vector {
    
    let magnitude = magnitude(start, end);

    return Vector{start, end, magnitude};
}

fn magnitude(start: coords::Coord , end: coords::Coord) -> f64 {
    let (mut x,mut y) = coords::Coord::to_tuple(end - start);
    x = x * x;
    y = y * y;

    let sum = x + y;
    let magnitude = sum.sqrt();
    return magnitude;
}

// math
impl ops::Add<Vector> for Vector{
    type Output = Vector;
    
    fn add(self: Vector, rhs: Vector) -> Vector {
        let start = self.start + rhs.start;
        let end = self.end + rhs.end;
        let magnitude = magnitude(start, end);

        return Vector{start, end, magnitude};
    }
}

impl ops::Sub<Vector> for Vector{
    type Output = Vector;
    
    fn sub(self: Vector, rhs: Vector) -> Vector {
        let start = self.start - rhs.start;
        let end = self.end - rhs.end;
        let magnitude = magnitude(start, end);

        return Vector{start, end, magnitude};

    }
}

impl ops::Mul<Vector> for Vector{
    type Output = Vector;
    
    fn mul(self: Vector, rhs: Vector) -> Vector {
        let start = self.start * rhs.start;
        let end = self.end * rhs.end;
        let magnitude = magnitude(start, end);

        return Vector{start, end, magnitude};

    }
}

impl ops::Div<Vector> for Vector{
    type Output = Vector;
    
    fn div(self: Vector, rhs: Vector) -> Vector {
        let start = self.start / rhs.start;
        let end = self.end / rhs.end;
        let magnitude = magnitude(start, end);

        return Vector{start, end, magnitude};
    }
}

impl Vector {
    pub fn to_vec(self : Vector) -> Vec<coords::Coord> {
        return vec![self.start, self.end];
    }
    
    pub fn to_tuple(self : Vector) -> (coords::Coord, coords::Coord, f64) {
        return (self.start, self.end, self.magnitude);
        
    }

    pub fn split(self: Vector) -> (coords::Coord, coords::Coord, f64) {
        return self.to_tuple();
    }
    
    pub fn get_magnitude(self : Vector) -> f64 {
        let mag_coord= self.end - self.start;
        let (mut x,mut y) = mag_coord.to_tuple();
        x = x * x;
        y = y * y;
    
        let sum = x + y;
        let magnitude = sum.sqrt();
        return magnitude;
    }
    
}
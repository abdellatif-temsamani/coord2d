use crate::coords;
use std::ops;

#[derive(Clone, Copy , Debug)]
pub struct Vector {
    start: coords::Coord,
    end: coords::Coord,
}


pub fn new(start: coords::Coord, end: coords::Coord) -> Vector {
    
    return Vector{start, end};
}

// math
impl ops::Add<Vector> for Vector{
    type Output = Vector;
    
    fn add(self: Vector, rhs: Vector) -> Vector {
        let start = self.start + rhs.start;
        let end = self.end + rhs.end;

        return Vector{start, end};
    }
}

impl ops::Sub<Vector> for Vector{
    type Output = Vector;
    
    fn sub(self: Vector, rhs: Vector) -> Vector {
        let start = self.start - rhs.start;
        let end = self.end - rhs.end;

        return Vector{start, end};
    }
}

impl ops::Mul<Vector> for Vector{
    type Output = Vector;
    
    fn mul(self: Vector, rhs: Vector) -> Vector {
        let start = self.start * rhs.start;
        let end = self.end * rhs.end;

        return Vector{start, end};
    }
}

impl ops::Div<Vector> for Vector{
    type Output = Vector;
    
    fn div(self: Vector, rhs: Vector) -> Vector {
        let start = self.start / rhs.start;
        let end = self.end / rhs.end;

        return Vector{start, end};
    }
}

impl Vector {
    pub fn to_vec(vector : Vector) -> Vec<coords::Coord> {
        return vec![vector.start, vector.end];
    }
    
    pub fn to_tuple(vector : Vector) -> (coords::Coord, coords::Coord) {
        return (vector.start, vector.end);
        
    }

    pub fn split(vector: Vector) -> (coords::Coord, coords::Coord) {
        return self::Vector::to_tuple(vector);
    }
}
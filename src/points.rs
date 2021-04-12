use crate::coords;
use std::ops;


#[derive(Clone, Copy , Debug)]
pub struct Point {
    start: coords::Coord,
    end: coords::Coord,
}


pub fn new(start: coords::Coord, end: coords::Coord) -> Point {
    return Point {start, end};
}

// math
impl ops::Add<Point> for Point{
    type Output = Point;
    
    fn add(self: Point, rhs: Point) -> Point {
        let start = self.start + rhs.start;
        let end = self.end + rhs.end;

        return Point{start, end};
    }
}

impl ops::Sub<Point> for Point{
    type Output = Point;
    
    fn sub(self: Point, rhs: Point) -> Point {
        let start = self.start - rhs.start;
        let end = self.end - rhs.end;

        return Point{start, end};
    }
}

impl ops::Mul<Point> for Point{
    type Output = Point;
    
    fn mul(self: Point, rhs: Point) -> Point {
        let start = self.start * rhs.start;
        let end = self.end * rhs.end;

        return Point{start, end};
    }
}

impl ops::Div<Point> for Point{
    type Output = Point;
    
    fn div(self: Point, rhs: Point) -> Point {
        let start = self.start / rhs.start;
        let end = self.end / rhs.end;

        return Point{start, end};
    }
}
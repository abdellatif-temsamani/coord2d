use crate::coords;
use std::ops;


#[derive(Clone, Copy , Debug)]
pub struct Line {
    start: coords::Coord,
    end: coords::Coord,
}


pub fn new(start: coords::Coord, end: coords::Coord) -> Line {
    return Line{start, end};
}

// math
impl ops::Add<Line> for Line{
    type Output = Line;
    
    fn add(self: Line, rhs: Line) -> Line {
        let start = self.start + rhs.start;
        let end = self.end + rhs.end;

        return Line{start, end};
    }
}

impl ops::Sub<Line> for Line{
    type Output = Line;
    
    fn sub(self: Line, rhs: Line) -> Line {
        let start = self.start - rhs.start;
        let end = self.end - rhs.end;

        return Line{start, end};
    }
}

impl ops::Mul<Line> for Line{
    type Output = Line;
    
    fn mul(self: Line, rhs: Line) -> Line {
        let start = self.start * rhs.start;
        let end = self.end * rhs.end;

        return Line{start, end};
    }
}

impl ops::Div<Line> for Line{
    type Output = Line;
    
    fn div(self: Line, rhs: Line) -> Line {
        let start = self.start / rhs.start;
        let end = self.end / rhs.end;

        return Line{start, end};
    }
}

impl Line {
    pub fn to_vec(line : Line) -> Vec<coords::Coord> {
        return vec![line.start, line.end];
    }
    
    pub fn to_tuple(line : Line) -> (coords::Coord, coords::Coord) {
        return (line.start, line.end);
        
    }

    pub fn split(point: Line) -> (coords::Coord, coords::Coord) {
        return self::Line::to_tuple(point);
    }
}
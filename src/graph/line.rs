use super::point::Point;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Line {
    a: Point,
    b: Point
}

impl Line {
    pub fn new(a: Point, b: Point) -> Line {
        Line {
            a,
            b
        }
    }
}
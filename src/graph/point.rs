use std::fmt::Debug;

pub enum Quadrant {
    One,
    Two,
    Three,
    Four,
    None
}

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x,
            y
        }
    }

    pub fn get_quadrant(&self) -> Quadrant {
        if self.x == 0 || self.y == 0 {
            return Quadrant::None
        }

        match self.x {
            > 4 => Quadrant::Four
        }
    }
}
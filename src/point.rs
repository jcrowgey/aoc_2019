use std::cmp::Ordering;

#[derive(Debug,Hash,Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn up(&self) -> Self { // returns the point above
        Point{
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down(&self) -> Self {
        Point{
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn left(&self) -> Self {
        Point{
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Point{
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.dist()).cmp(&(other.dist()))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


use super::boards; 

#[derive(Debug)]
pub struct Ship {
    pub start: (String, usize),
    pub length: usize,
    pub direction: Direction,
}

impl Ship {
    pub fn new(start: (String, usize), length: usize, direction: Direction) -> Self{
        Self {
            start,
            length,
            direction,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    NorthSouth,
    WestEast,
}

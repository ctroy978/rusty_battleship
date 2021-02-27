
use super::boards; 

#[derive(Debug, PartialEq)]
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


pub fn build_ship(ocean: &mut boards::Board, start: (String, usize), 
                   length: usize, direction: Direction ) -> Option<Ship>{

        let ship = Ship::new((start.0, start.1), length, direction); 

        if ocean.fit_check(&ship){
            return Some(ship);
        }else{
            return None;
        }

    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    NorthSouth,
    WestEast,
}

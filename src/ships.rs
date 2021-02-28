use std::io;

use super::boards; 

#[derive(Debug, PartialEq)]
pub struct Ship {
    pub fleet: usize,
    pub start: (String, usize),
    pub length: usize,
    pub direction: Direction,
}

impl Ship {
    pub fn new(fleet: usize, start: (String, usize), length: usize, direction: Direction) -> Self{
        Self {
            fleet,
            start,
            length,
            direction,
        }
    }


    ///Checks if ship fits in ocean before adding to fleet
    pub fn build_ship(ocean: &mut boards::Board, fleet: usize, start: (String, usize), 
                   length: usize, direction: Direction ) -> Option<Ship>{

        let ship = Ship::new(fleet, (start.0, start.1), length, direction); 

        if ocean.fit_check(&ship){
            return Some(ship);
        }else{
            return None;
        }

    }

    ///Validates user input on ship type
    pub fn valid_len(fleets: &mut Vec<Ship>) -> usize{

        let carrier: usize = 8;
        let battleship: usize = 7;
        let destroyer: usize = 5;
        let cruiser: usize = 4;
        let pt_boat: usize = 2;

        let ship_size = match fleets.len(){
            0 => carrier,
            1 => battleship,
            2 => battleship,
            3 => destroyer,
            4 => cruiser,
            _ => pt_boat,
        };
        let ship_type = match ship_size{
            8 => "Carrier -len: 8",
            7 => "Battleship -len: 7",
            5 => "Destroyer -len: 5",
            4 => "Cruiser -len: 3",
            2 => "PT Boat -len: 2",
            _ => "unknown",
        };
        println!("Prepare to place a {}", ship_type);
        ship_size
    }

    ///validates user input for direction of ship
    pub fn valid_dir() -> Direction{

        loop{
            let mut direction = String::new();
            println!("Input ship orientation -NS or -EW");
            io::stdin()
                .read_line(&mut direction)
                .expect("Issue reading direction input");

            direction = direction.trim().to_uppercase().to_string();

            let dir = match &direction[..]{
                "NS" => Direction::NorthSouth, 
                "-NS" => Direction::NorthSouth,
                "NORTHSOUTH" => Direction::NorthSouth,
                "EW" => Direction::WestEast,
                "-EW" => Direction::WestEast,
                "EASTWEST" => Direction::WestEast,
                _ => {
                    println!("Not a valid entry");
                    continue;
                }
            }; 
            return dir;
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    NorthSouth,
    WestEast,
}

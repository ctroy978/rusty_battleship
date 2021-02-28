mod boards;
mod ships;


fn main() {
    //2d array &str
    let mut ocean = boards::Board::new(20, 20);
    
    let mut fleets: Vec<ships::Ship> = Vec::new();

    let mut total_ships = 6;
    
    while total_ships > 0{

        rusty_battleship::clear_screen();

        ocean.draw();
        let fleet = 1;
        let length = ships::Ship::valid_len(&mut fleets);
        let row = ocean.valid_row(); 
        let col = ocean.valid_col();
        let direction = ships::Ship::valid_dir(); 

        match ships::Ship::build_ship(&mut ocean, fleet, (row, col), length, direction){
            Some(s) => {
                println!("Ship placed in fleet");
                ocean.place_ship(&s);
                fleets.push(s);
            },
            None => {println!("Problem with ship. It was not added to fleet.")},
        };
        total_ships -= 1;
    }

    println!("{:?}", fleets);
    ocean.draw();
}

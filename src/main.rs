mod boards;
mod ships;


fn main() {
    //2d array &str
    let mut ocean = boards::Board::new(20, 20);
    
    let mut player1: Vec<ships::Ship> = Vec::new();
    let mut player2: Vec<ships::Ship> = Vec::new();

    let mut total_ships = 5;
    
    while total_ships > 0{

        ocean.draw();
        let row = ocean.valid_row(); 
        let col = ocean.valid_col();
        let length = 5;
        let direction = ships::Direction::WestEast;

        let ship = match ships::Ship::build_ship(&mut ocean, (row, col), length, direction){
            Some(s) => {
                println!("Ship placed in fleet");
                ocean.place_ship(&s);
                player1.push(s);
            },
            None => {println!("Problem with ship. It was not added to fleet.")},
        };
        total_ships -= 1;
    }

    println!("{:?}", player1);
    ocean.draw();
}

mod boards;
mod ships;


fn main() {
    //2d array &str
    let mut ocean = boards::Board::new(20, 20);
    
    let mut player1: Vec<ships::Ship> = Vec::new();
    let mut player2: Vec<ships::Ship> = Vec::new();

    let mut total_ships = 5;
    
    while total_ships > 0{

        let row = "B".to_string();
        let col = 4;
        let length = 5;
        let direction = ships::Direction::WestEast;
        let ship = ships::Ship::build_ship(&mut ocean, (row, col), length, direction);
        if ship != None{
            let ship = ship.unwrap();
            ocean.place_ship(&ship);
            player1.push(ship);
        }
        total_ships -= 1;
    }

    //let player1 = ships::Ship::new(("C".to_string(),10), 4, ships::Direction::WestEast); 
    //v.place_ship(player1);

    println!("{:?}", player1);
    ocean.draw();
}

mod boards;
mod ships;


fn main() {
    //2d array &str
    let mut v = boards::Board::new(20, 20);

    let player1 = ships::Ship::new(("C".to_string(),10), 4, ships::Direction::WestEast); 
    v.place_ship(player1);
    let player2 = ships::Ship::new(("A".to_string(),5), 7, ships::Direction::NorthSouth); 
    v.place_ship(player2);
    
    let x = v.ship_fits(("B".to_string(), 2), 10, ships::Direction::NorthSouth);
    println!("{:?}", x);
    v.draw();
}


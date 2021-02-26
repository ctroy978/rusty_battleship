mod boards;
mod ships;


fn main() {
    //2d array &str
    let mut ocean = boards::Board::new(20, 20);
    
    let mut player1: Vec<ships::Ship> = Vec::new();

    let mut total = 5;
    
    while total > 0{

        let ship = ships::Ship::new(("C".to_string(),10), 4, ships::Direction::WestEast); 

        if ocean.fit_check(&ship){
            println!("ship was placed");
            ocean.place_ship(&ship);
            player1.push(ship); 
        }else{
            println!("ship not placed");
        }
        total -= 1;
    }

    //let player1 = ships::Ship::new(("C".to_string(),10), 4, ships::Direction::WestEast); 
    //v.place_ship(player1);

    println!("{:?}", player1);
    ocean.draw();

}


fn f(v: i32) -> Option<String>{
    let x = String::from("HI");
    if v < 5{
       return None; 
    }
    Some(x)
}


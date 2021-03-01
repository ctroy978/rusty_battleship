mod boards;
mod ships;


fn main() {
    //2d array &str
    
    let mut ocean1 = boards::Board::new(20, 20);
    let mut fleets1: Vec<ships::Ship> = Vec::new();
    
    let mut ocean2 = boards::Board::new(20, 20);
    let mut fleets2: Vec<ships::Ship> = Vec::new();
    
    let mut player_1 = Player{
        player: String::from("Player One"),
        ocean: ocean1,
        fleet: fleets1,
    };

    let mut player_2 = Player{
        player: String::from("Player Two"),
        ocean: ocean2,
        fleet: fleets2,
    };

    setup(&player_1.player, &mut player_1.ocean, &mut player_1.fleet);
    setup(&player_2.player, &mut player_2.ocean, &mut player_2.fleet);
}

///
fn setup(player: &String, ocean: &mut boards::Board, fleets: &mut Vec<ships::Ship>){
    let mut total_ships = 6;
    
    while total_ships > 0{

        rusty_battleship::clear_screen();

        ocean.draw();
        println!("");
        println!("Build your fleet -- {} --", player);

        let fleet = 1;
        let length = ships::Ship::valid_len(fleets);
        let row = ocean.valid_row(); 
        let col = ocean.valid_col();
        let direction = ships::Ship::valid_dir(); 

        match ships::Ship::build_ship(ocean, fleet, (row, col), length, direction){
            Some(s) => {
                println!("Ship placed in fleet");
                ocean.place_ship(&s);
                fleets.push(s);
            },
            None => {println!("Problem with ship. It was not added to fleet.")},
        };
        total_ships -= 1;
    }
}

struct Player{
    player: String,
    ocean:  boards::Board,
    fleet: Vec<ships::Ship>,
}

use super::ships;

pub struct Board {
    ///The board is a two dimensional vector filled with "~~"
    pub board: Vec<Vec<String>>,
    row: usize,
    col: usize,
}
impl Board {
    ///returns the board filled with empty water
    ///
    ///#arguments
    ///
    /// row -the number of rows you want (letters)
    /// col -the number of col you want (numbered)
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            //board: [["~"; 20]; 20],
            board: vec![vec![String::from("~~|"); col]; row],
            row,
            col,
        }
    }

    /// Draw the board and fill with water.
    /// some cheap printing involved to line up board with numbers.
    pub fn draw(&self) {

        let letter = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

        let mut alpha = letter.chars();

        //print numbers across top
        print!("  ");
        for (i, _) in self.board[0].iter().enumerate() {
            print!("{:02}|", i);
        }
        println!("");

        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if j == 0 {
                    //print letters down left side
                    print!(" {}", alpha.next().unwrap());
                }
                print!("{}", self.board[i][j]);
            }
            println!("");
        }
    }
    /// place the ship onto the board
    pub fn place_ship(& mut self, ship: &ships::Ship){
        //put ship on board
        let (letter, _) = &ship.start;
        let (_, col) = ship.start;

        use rusty_battleship::alpha_to_digit;
        let row = alpha_to_digit(&letter);


        match ship.direction{
            ships::Direction::NorthSouth => {
                for i in row .. row+ship.length{
                    self.board[i][col] = "ss|".to_string();
                }
            }

            ships::Direction::WestEast =>{
                for i in col..col+ship.length{
                    self.board[row][i] = "ss|".to_string();
                }
            }
        }
    }

    ///check if ship can fit on board
    pub fn fit_check(&self, ship: &ships::Ship)-> bool{
        let water = String::from("~~|");
        let (letter, _) = &ship.start;
        let (_, col) = ship.start;


        //use rusty_battleship::alpha_to_digit;
        let row = rusty_battleship::alpha_to_digit(&letter);

        match ship.direction{
            ships::Direction::NorthSouth =>{
                if row + ship.length > self.row{
                    return false;
                }
                for i in row..row + ship.length{
                    if self.board[i][col] != water{
                        return false;
                    } 
                }
            }

            ships::Direction::WestEast =>{
                if col + ship.length > self.col{
                    return false;
                }
                for i in col..col + ship.length{
                    if self.board[row][i] != water{
                        return false;
                    } 
                }
            }
        }
        //ship fits in ocean
        true
    }
}

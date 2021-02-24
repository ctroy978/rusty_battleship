use super::ships;

pub struct Board {
    ///The board is a two dimensional vector filled with "~~"
    pub board: Vec<Vec<String>>,
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
    pub fn place_ship(& mut self, ship: ships::Ship){
        //pub ship on board
        let (letter, col) = ship.start;

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

    pub fn ship_fits(&self, start: (String,usize), length: usize, direction: ships::Direction)-> bool{
        //empty space on board
        let empty_water = String::from("~~|"); 
        let mut fits = true;
        
        let (letter, col) = start;

        use rusty_battleship::alpha_to_digit;
        let row = alpha_to_digit(&letter);

        match direction{
            ships::Direction::NorthSouth => {
                for i in row .. row+length{
                    if self.board[i][col] != empty_water{
                        fits = false;
                    } 
                }
            }
            ships::Direction::WestEast=> {
                for i in col..col_ship.length{
                    if let check = self.board[row][i]{

                    }
                }
            }
        }
        fits 
    }
}

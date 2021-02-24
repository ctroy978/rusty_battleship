fn main() {
    //2d array &str
    let mut v = Board::new(20, 20);

    let player1 = Ship::new(("C".to_string(),10), 4, Direction::WestEast); 
    v.place_ship(player1);
    let player2 = Ship::new(("A".to_string(),5), 7, Direction::NorthSouth); 
    v.place_ship(player2);
    v.draw();
}

struct Board {
    ///The board is a two dimensional vector filled with "~~"
    board: Vec<Vec<String>>,
}

impl Board {
    ///returns the board filled with empty water
    ///
    ///#arguments
    ///
    /// row -the number of rows you want (letters)
    /// col -the number of col you want (numbered)
    fn new(row: usize, col: usize) -> Self {
        Self {
            //board: [["~"; 20]; 20],
            board: vec![vec![String::from("~~|"); col]; row],
        }
    }

    /// Draw the board and fill with water.
    /// some cheap printing involved to line up board with numbers.
    fn draw(&self) {

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
    fn place_ship(& mut self, ship: Ship){
        //pub ship on board
        let (letter, col) = ship.start;

        use rusty_battleship::alpha_to_digit;
        let row = alpha_to_digit(&letter);


        match ship.direction{
            Direction::NorthSouth => {
                for i in row .. row+ship.length{
                    self.board[i][col] = "ss|".to_string();
                }
            }

            Direction::WestEast =>{
                for i in col..col+ship.length{
                    self.board[row][i] = "ss|".to_string();
                }
            }
        }


    }
}

struct Ship {
    start: (String, usize),
    length: usize,
    direction: Direction,
}

impl Ship {
    fn new(start: (String, usize), length: usize, direction: Direction) -> Self {
        Self {
            start,
            length,
            direction,
        }
    }
}

enum Direction {
    NorthSouth,
    WestEast,
}

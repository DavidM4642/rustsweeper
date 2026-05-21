use std::fmt;
// fmt will help draw the grid
use rand::Rng;
// rng generator


#[derive(Clone)]
struct Cell {
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
    nearby_crabs: u8,
}
// defining the cells and their states.

impl Cell {
    fn new() -> Self {
        Cell {
            is_mine: false,
            is_revealed: false,
            is_flagged: false,
            nearby_crabs: 0,
        }
    }
}
// creates the cell with nothing in it.
// impl is what we use to tell rust what the struct will do.

// Creating the actual board//grid
struct Board {
    cell: Vec<Vec<Cell>>, // a list of list of cells
    cols: usize,
    rows: usize,
    total_crabs: usize,
}

impl Board {
    fn new(rows: usize, cols: usize, total_crabs: usize) -> Self {
        Board {
            cell: vec![vec![Cell::new(); cols]; rows],
            rows,
            cols,
            total_crabs,
        }
    }
    fn place_crabs(&mut self){
// setting up the rng generator, and the crab counter
        let mut rng = rand::thread_rng();
        let mut placed = 0;
// while loop that first looks for a random spot on the board
// once it finds the spot it places a mine (setting is_mine to true)
// and adds 1 to the counter
        while placed < self.total_crabs {
            let row = rng.gen_range(0..self.rows);
            let col = rng.gen_range(0..self.cols);

            if !self.cell[row][col].is_mine{
                self.cell[row][col].is_mine = true;
                placed += 1;
            }
        }
    }
    fn calculate_nearby_crabs(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.cell[row][col].is_mine {
                    continue;
                }

                let mut count = 0;

                for dr in -1i32..=1 {
                    for dc in -1i32..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let neighbor_row = row as i32 + dr;
                        let neighbor_col = col as i32 + dc;

                        if neighbor_row >= 0
                            && neighbor_row < self.rows as i32
                            && neighbor_col >= 0
                            && neighbor_col < self.cols as i32
                        {
                            if self.cell[neighbor_row as usize][neighbor_col as usize].is_mine {
                                count += 1;
                            }
                        }
                    }
                }

                self.cell[row][col].nearby_crabs = count;
            }
        }
    }
// Drawing the actual board
    fn draw (&self){
        for row in 0..self.rows {
            for col in 0..self.cols {
            if !self.cell[row][col].is_revealed{
                print!("#")
            }else if self.cell[row][col].is_mine{
                print!("💣")
            }else if self.cell[row][col].nearby_crabs > 0{
                print!("{}", self.cell[row][col].nearby_crabs)
            }else {
                print!(" ")
            }
            }
            println!();
        }

    }
}

fn main() {
    let mut board = Board::new(9, 9, 10);
    board.place_crabs();
    board.calculate_nearby_crabs();
    board.draw();
    println!("Crabs and numbers calculated!");
}

// struct is the list the thing is made of
// impl/fn is the instructions of what the thing does
// Board::new(9, 9, 10) is actually making the thing

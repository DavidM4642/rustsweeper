use rand::Rng;

use crate::cell::Cell;
pub struct Board {
    pub cell: Vec<Vec<Cell>>, // a list of list of cells
    pub cols: usize,
    pub rows: usize,
    pub total_crabs: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize, total_crabs: usize) -> Self {
        Self {
            cell: vec![vec![Cell::new(); cols]; rows],
            rows,
            cols,
            total_crabs,
        }
    }

    pub fn place_crabs(&mut self) {
        // setting up the rng generator, and the crab counter
        let mut rng = rand::thread_rng();
        let mut placed = 0;
        // while loop that first looks for a random spot on the board
        // once it finds the spot it places a mine (setting is_mine to true)
        // and adds 1 to the counter
        while placed < self.total_crabs {
            let row = rng.gen_range(0..self.rows);
            let col = rng.gen_range(0..self.cols);

            if !self.cell[row][col].is_mine {
                self.cell[row][col].is_mine = true;
                placed += 1;
            }
        }
    }

    pub fn calculate_nearby_crabs(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.cell[row][col].is_mine {
                    continue;
                }

                let mut count = 0;

                // Board edges have fewer neighbors, so clamp the 3x3 scan window before
                // indexing.
                let first_neighbor_row = row.saturating_sub(1);
                let last_neighbor_row = row.saturating_add(1).min(self.rows - 1);
                let first_neighbor_col = col.saturating_sub(1);
                let last_neighbor_col = col.saturating_add(1).min(self.cols - 1);

                for neighbor_row in first_neighbor_row..=last_neighbor_row {
                    for neighbor_col in first_neighbor_col..=last_neighbor_col {
                        if neighbor_row == row && neighbor_col == col {
                            continue;
                        }

                        if self.cell[neighbor_row][neighbor_col].is_mine {
                            count += 1;
                        }
                    }
                }

                self.cell[row][col].nearby_crabs = count;
            }
        }
    }

    // Drawing the actual board
    pub fn draw(&self) {
        // print column numbers across the top
        print!("  ");
        for col in 0..self.cols {
            print!("{} ", col + 1);
        }
        println!();
        // print each row with a letter label
        for row in 0..self.rows {
            let letter = u8::try_from(row)
                .ok()
                .and_then(|row_offset| b'A'.checked_add(row_offset))
                .map_or('?', char::from);
            print!("{letter} ");
            for col in 0..self.cols {
                let cell = &self.cell[row][col];
                if cell.is_flagged {
                    print!("🚩");
                } else if !cell.is_revealed {
                    print!("# ");
                } else if cell.is_mine {
                    print!("🦀");
                } else if cell.nearby_crabs > 0 {
                    print!("{} ", cell.nearby_crabs);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    // reveal requested cell
    pub fn reveal_cell(&mut self, row: usize, col: usize) {
        self.cell[row][col].is_revealed = true;
    }

    // flag cell is requested
    pub fn toggle_flag(&mut self, row: usize, col: usize) {
        self.cell[row][col].is_flagged = !self.cell[row][col].is_flagged;
    }

    pub fn is_mine_hit(&self) -> bool {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.cell[row][col].is_revealed && self.cell[row][col].is_mine {
                    return true;
                }
            }
        }

        false
    }

    #[expect(dead_code, reason = "will be used later")]
    pub fn is_won(&self) -> bool {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = &self.cell[row][col];
                if !cell.is_mine && !cell.is_revealed {
                    return false;
                }
            }
        }
        true
    }

    #[expect(dead_code, reason = "will be used later")]
    pub fn reveal_all_mines(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.cell[row][col].is_mine {
                    self.cell[row][col].is_revealed = true;
                }
            }
        }
    }
}

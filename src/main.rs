use std::fmt;
// fmt will help draw the grid

#[derive(Clone)]
struct Cell {
    is_mine: bool,
    is_revealed: bool,
    is_flagged: bool,
    nearby_crabs: u8,
}
//defining the cells and their states.

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
//creates the cell with nothing in it.
// impl is what we use to tell rust what the struct will do.

//Creating the actual board//grid

struct Board {
    cell: Vec<Vec<Cell>>, //a list of list of cells
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
}

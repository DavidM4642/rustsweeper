#[derive(Clone)]
pub struct Cell {
    pub is_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub nearby_crabs: u8,
}
// defining the cells and their states.

impl Cell {
    pub const fn new() -> Self {
        Self {
            is_mine: false,
            is_revealed: false,
            is_flagged: false,
            nearby_crabs: 0,
        }
    }
}

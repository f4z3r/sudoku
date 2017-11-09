//! Game logic

/// Size of game board
const SIZE: usize = 9;

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[Cell; SIZE]; SIZE],
}

impl Gameboard {
    /// Creates a new gameboard
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[Cell::new(); SIZE]; SIZE],
        }
    }

    /// Gets the character at cell location.
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]].value {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None,
        })
    }

    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        if self.cells[ind[1]][ind[0]].editable == true {
            self.cells[ind[1]][ind[0]].value = val;
        }
    }

    /// Retrieve editable information from cell.
    pub fn is_editable(&self, ind: [usize; 2]) -> bool {
        return self.cells[ind[1]][ind[0]].editable;
    }

    /// Sets whether a cell is editable.
    pub fn set_editable(&mut self, ind: [usize; 2], editable: bool) {
        self.cells[ind[1]][ind[0]].editable = editable;
    }

    /// Initialises the gameboard to a solvable sudoku.
    pub fn init(&mut self) {
        use rand;
        use rand::distributions::{IndependentSample, Range};

        let between = Range::new(0, 9);
        let mut rng = rand::thread_rng();

        loop {
            for idx in 0..80 {
                let pos_x = between.ind_sample(&mut rng);
                let pos_y = between.ind_sample(&mut rng);
                let value = (between.ind_sample(&mut rng) + 1) as u8;

                if !self.conflict(&[pos_x, pos_y], value) {
                    self.set([pos_x, pos_y], value);
                    self.set_editable([pos_x, pos_y], false);
                }
            }
            if self.solve() { break; }
        }
    }

    /// Checks for solution via brute force backtracking.
    pub fn solve(&mut self) -> bool {
        // Create a list of all editable cells.
        let mut empty_cells = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if self.is_editable([i, j]) {
                    empty_cells.push([i, j]);
                }
            }
        }
        self.backtrack(0, &empty_cells)
    }

    /// Recursive function for solution checking.
    fn backtrack(&mut self, ind: usize, empty_cells: &Vec<[usize; 2]>) -> bool {
        if ind > (empty_cells.len() - 1) { return true; }
        for value in 1..10 {
            if !self.conflict(&empty_cells[ind], value) {
                self.set(empty_cells[ind], value);
                if self.backtrack(ind + 1, empty_cells) { return true; }
            }
        }
        self.set(empty_cells[ind], 0);
        return false;
    }

    /// Function checking for conflicts for a given value.
    fn conflict(&self, ind: &[usize; 2], value: u8) -> bool {
        for i in 0..9 {
            if self.cells[ind[0]][i].value() == value { return true; }
            if self.cells[i][ind[1]].value() == value { return true; }
        }
        let box_row = (ind[0]/3)*3;
        let box_col = (ind[1]/3)*3;
        for row in 0..3 {
            for col in 0..3 {
                if self.cells[box_row + row][box_col + col].value() == value {
                    return true;
                }
            }
        }
        return false;
    }
}

/// Stores the cell's information.
#[derive(Copy, Clone)]
pub struct Cell {
    /// The value of that cell.
    value: u8,
    /// Whether the cell is editable.
    editable: bool,
}

impl Cell {
    /// Creates a new cell.
    fn new() -> Cell {
        Cell {
            value: 0,
            editable: true,
        }
    }

    pub fn value(&self) -> u8 {
        return self.value;
    }
}

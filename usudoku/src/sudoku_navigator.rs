use crate::sudoku_board::{CellPosition, SudokuBoard};

impl SudokuBoard {
    pub fn navigator(&mut self) -> SudokuNavigator {
        SudokuNavigator {
            current: Some(self.first_cell()),
        }
    }
}

pub struct SudokuNavigator {
    current: Option<CellPosition>,
}

impl SudokuNavigator {
    pub fn current(&self) -> Option<CellPosition> {
        self.current
    }

    pub fn forward(&mut self, board: &SudokuBoard) -> Option<CellPosition> {
        self.current = self.current.and_then(|c| board.next_cell(c));
        self.current
    }

    pub fn backwards(&mut self, board: &SudokuBoard) -> Option<CellPosition> {
        self.current = self.current.and_then(|c| board.previous_cell(c));
        self.current
    }
}

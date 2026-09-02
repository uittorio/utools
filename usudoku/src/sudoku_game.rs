use crate::sudoku_board::SudokuSize;
use crate::{
    sudoku_board::{CellPosition, SudokuBlock, SudokuBoard},
    sudoku_clues::{SudokuDifficulty, min_17},
    sudoku_solution::SudokuSolution,
    sudoku_solution_generator::SudokuSolutionBacktrackingGenerator,
    sudoku_value::SudokuValue,
};

pub struct SudokuGame {
    #[allow(dead_code)]
    size: SudokuSize,
    solution: SudokuSolution,
    clues: SudokuBoard,
    board: SudokuBoard,
}

pub enum SudokuError {
    WrongValue,
    TryToChangeClue,
}

impl SudokuGame {
    pub fn new() -> Self {
        let size = SudokuSize::NineByNine;
        let solution = SudokuSolutionBacktrackingGenerator::new(size);
        let clues = min_17(&solution, SudokuDifficulty::Easy, size);

        let board = SudokuBoard::from(&clues);

        SudokuGame {
            clues,
            size,
            solution,
            board,
        }
    }

    pub fn is_clue(&self, cell_position: CellPosition) -> bool {
        self.clues.blocks[cell_position.block][cell_position.cell]
            .value
            .is_some()
    }

    pub fn is_correct(&self, position: CellPosition, value: SudokuValue) -> bool {
        self.solution.blocks[position.block][position.cell].value == value
    }

    pub fn blocks(&self) -> &Vec<SudokuBlock> {
        &self.board.blocks
    }

    pub fn set(&mut self, position: CellPosition, value: SudokuValue) -> Result<(), SudokuError> {
        if self.is_clue(position) {
            return Err(SudokuError::TryToChangeClue);
        }

        self.board.set(position, value);

        if self.solution.blocks[position.block][position.cell].value == value {
            Ok(())
        } else {
            return Err(SudokuError::WrongValue);
        }
    }

    pub fn annotate(&mut self, position: CellPosition, value: SudokuValue) {
        if self.board.has_annotation(position, value) {
            self.board.remove_annotation(position, value);
        } else {
            self.board.add_annotation(position, value);
        }
    }

    pub fn remove_annotations(&mut self, position: CellPosition) {
        self.board.clear_annotations(position);
    }

    pub fn empty(&mut self, position: CellPosition) -> Result<(), SudokuError> {
        if self.is_clue(position) {
            return Err(SudokuError::TryToChangeClue);
        }
        Ok(self.board.empty(position))
    }

    pub fn next_up(&mut self, current_cell: CellPosition) -> CellPosition {
        self.board.next_up(current_cell)
    }

    pub fn next_down(&mut self, current_cell: CellPosition) -> CellPosition {
        self.board.next_down(current_cell)
    }

    pub fn next_right(&mut self, current_cell: CellPosition) -> CellPosition {
        self.board.next_right(current_cell)
    }

    pub fn next_left(&mut self, current_cell: CellPosition) -> CellPosition {
        self.board.next_left(current_cell)
    }
}

use crate::sudoku_value::SudokuValue;

pub struct SudokuSolution {
    pub blocks: Vec<Vec<SudokuSolutionCell>>,
}

pub struct SudokuSolutionCell {
    pub value: SudokuValue,
}

use std::collections::HashSet;

use rand::random_range;

use crate::{
    sudoku_backtracking_generator::SudokuBacktrackingGenerator,
    sudoku_board::{CellPosition, SudokuBoard},
};

pub fn sudoku_generator() -> SudokuBoard {
    let mut sudoku = SudokuBacktrackingGenerator::new();

    sudoku.clear_all_annotations();
    remove_random_values(&mut sudoku, 20);

    sudoku
}

// TODO
// in reality a valid sudoku should only have one solution and we have to enforce that.
// bY removing cells by percentage there a risk that the grid will have more than one solution
// According to wikipedia the boards are defined by the number of clues instead to enfore a single solution
// https://en.wikipedia.org/wiki/Mathematics_of_Sudoku
pub fn remove_random_values(board: &mut SudokuBoard, percentage: usize) {
    let cells_to_empty = board.total_cells() * percentage / 100;

    let mut values = HashSet::new();

    while values.len() < cells_to_empty {
        values.insert(random_range(0..81));
    }

    for value in values {
        let block = value / 9;
        let cell = value % 9;

        board.empty(CellPosition { block, cell });
    }
}

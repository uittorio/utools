use std::collections::HashSet;

use rand::random_range;

use crate::{
    sudoku_board::{CellPosition, SudokuBoard, SudokuSize},
    sudoku_solution::SudokuSolution,
};

pub enum SudokuDifficulty {
    Easy,
    #[allow(dead_code)]
    Medium,
    #[allow(dead_code)]
    Hard,
}

pub fn min_17(
    solution: &SudokuSolution,
    difficulty: SudokuDifficulty,
    size: SudokuSize,
) -> SudokuBoard {
    // at the moment this is a super simplified version that just provides a random number of clues based on the difficulty
    // this solution has two problems that leads to the same output -> there might be more than one solution
    // 1) it could omit 2 digits completely leading to more than one solution
    // 2) it could not respect unavoidable sets.
    // rules
    // - at least 17 clues
    // - at least 8 out of 9 digits
    // https://arxiv.org/pdf/1201.0749 unavoidable sets
    // We could find a way to check if there is more than one solution
    //
    let clues_range = match difficulty {
        SudokuDifficulty::Easy => 35..45,
        SudokuDifficulty::Medium => 28..35,
        SudokuDifficulty::Hard => 17..28,
    };

    let clues = random_range(clues_range);

    let mut clues_board = SudokuBoard::new(size);

    let mut values = HashSet::new();

    while values.len() < clues {
        values.insert(random_range(0..81));
    }

    for value in values {
        let block = value / 9;
        let cell = value % 9;

        clues_board.set(
            CellPosition {
                block: block,
                cell: cell,
            },
            solution.blocks[block][cell].value,
        );
    }

    clues_board
}

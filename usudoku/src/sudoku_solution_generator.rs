use rand::random_range;

use crate::{
    sudoku_board::{CellPosition, SudokuBoard, SudokuSize},
    sudoku_solution::{SudokuSolution, SudokuSolutionCell},
    sudoku_value::SudokuValue,
};

pub struct SudokuSolutionBacktrackingGenerator {
    pub board: SudokuBoard,
}

impl SudokuSolutionBacktrackingGenerator {
    pub fn new(size: SudokuSize) -> SudokuSolution {
        let board = SudokuBoard::new(size);

        let mut generator = SudokuSolutionBacktrackingGenerator { board };

        generator.generate();

        generator.to_solution()
    }

    pub fn generate(&mut self) {
        self.board.annotate_all();

        let mut navigator = self.board.navigator();

        loop {
            match navigator.current() {
                Some(c) => match self.fill_or_backtrace_fill(c) {
                    true => {
                        if navigator.forward(&self.board).is_none() {
                            break;
                        }
                    }
                    false => {
                        if let Some(previous) = navigator.backwards(&self.board) {
                            self.board.empty(previous);
                        } else {
                            panic!("No backward cell found");
                        }
                    }
                },
                None => break,
            }
        }
    }

    fn fill_or_backtrace_fill(&mut self, cell_position: CellPosition) -> bool {
        let result = self.try_add_value(cell_position);

        match result {
            Some(value) => {
                self.board.set(cell_position, value);
                self.board.remove_annotation(cell_position, value);
                return true;
            }
            None => {
                self.board.empty(cell_position);
                self.board.full_annotation(cell_position);
                return false;
            }
        }
    }

    fn try_add_value(&mut self, cell_position: CellPosition) -> Option<SudokuValue> {
        let cell = self.board.get(cell_position);

        match cell.value {
            Some(v) => Some(v),
            None => {
                let remaining_to_try = &cell.annotations.iter().collect::<Vec<&SudokuValue>>();
                if remaining_to_try.len() == 0 {
                    return None;
                }

                let value_to_try = remaining_to_try[random_range(0..remaining_to_try.len())];
                let exist_in_row = self.board.exists_in_row(&value_to_try, cell_position);
                let exist_in_column = self.board.exists_in_column(&value_to_try, cell_position);
                let exist_in_block = self.board.exists_in_block(&value_to_try, cell_position);

                if exist_in_row || exist_in_column || exist_in_block {
                    self.board.remove_annotation(cell_position, *value_to_try);
                    return self.try_add_value(cell_position);
                }

                return Some(*value_to_try);
            }
        }
    }

    fn to_solution(&mut self) -> SudokuSolution {
        SudokuSolution {
            blocks: self
                .board
                .blocks
                .iter()
                .map(|b| {
                    b.into_iter()
                        .map(|c| SudokuSolutionCell {
                            value: c
                                .value
                                .expect("Cannot create a solution with an empty value"),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

use crate::sudoku_value::{SudokuValue, all_values};

pub type SudokuBlock = Vec<SudokuCell>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellPosition {
    pub block: usize,
    pub cell: usize,
}

#[derive(Debug)]
pub struct SudokuCell {
    pub position: CellPosition,
    pub value: Option<SudokuValue>,
    pub annotations: Vec<SudokuValue>,
}

impl SudokuCell {
    pub fn empty(block: usize, cell: usize) -> Self {
        SudokuCell {
            position: CellPosition { block, cell },
            annotations: vec![],
            value: None,
        }
    }
}

pub struct SudokuBoard {
    pub blocks: Vec<SudokuBlock>,
    size: SudokuSize,
}

pub enum SudokuSize {
    NineByNine,
}

impl SudokuBoard {
    pub fn new(size: SudokuSize) -> Self {
        let blocks: Vec<SudokuBlock> = match size {
            #[rustfmt::skip]
            SudokuSize::NineByNine => vec![
                block_9by9(0), block_9by9(1), block_9by9(2),
                block_9by9(3), block_9by9(4), block_9by9(5),
                block_9by9(6), block_9by9(7), block_9by9(8),
            ],
        };

        return SudokuBoard { blocks, size };
    }
}

impl SudokuBoard {
    pub fn total_cells(&self) -> usize {
        match self.size {
            SudokuSize::NineByNine => 81,
        }
    }

    pub fn annotate_all(&mut self) {
        for block in self.blocks.iter_mut() {
            for cell in block {
                cell.annotations = all_values();
            }
        }
    }

    pub fn clear_all_annotations(&mut self) {
        for block in self.blocks.iter_mut() {
            for cell in block {
                cell.annotations = vec![];
            }
        }
    }

    pub fn set(&mut self, position: CellPosition, value: SudokuValue) {
        self.blocks[position.block][position.cell].value = Some(value);
    }

    pub fn empty(&mut self, position: CellPosition) {
        self.blocks[position.block][position.cell].value = None;
    }

    pub fn remove_annotation(&mut self, cell_position: CellPosition, value: SudokuValue) {
        let sudoku_cell = &mut self.blocks[cell_position.block][cell_position.cell];
        let current_annotation = sudoku_cell
            .annotations
            .iter()
            .position(|v| *v == value)
            .expect(
                format!(
                    "Cannot remove a value that is not present {:?}",
                    cell_position
                )
                .as_str(),
            );

        sudoku_cell.annotations.remove(current_annotation);
    }

    pub fn full_annotation(&mut self, cell_position: CellPosition) {
        self.blocks[cell_position.block][cell_position.cell].annotations = all_values();
    }

    pub fn exists_in_row(&self, value: &SudokuValue, cell_position: CellPosition) -> bool {
        let block = cell_position.block;
        let cell = cell_position.cell;
        let blocks_to_check = match block {
            0..=2 => [0, 1, 2],
            3..=5 => [3, 4, 5],
            6..=8 => [6, 7, 8],
            _ => panic!("Invalid block index, {}", block),
        };

        let cells_to_check = match cell {
            0..=2 => [0, 1, 2],
            3..=5 => [3, 4, 5],
            6..=8 => [6, 7, 8],
            _ => panic!("Invalid cell index: {}", cell),
        };

        return self.exists(value, blocks_to_check, cells_to_check);
    }

    pub fn exists_in_column(&self, value: &SudokuValue, cell_position: CellPosition) -> bool {
        let block_i = cell_position.block;
        let cell_i = cell_position.cell;
        let blocks_to_check = match block_i {
            0 | 3 | 6 => [0, 3, 6],
            1 | 4 | 7 => [1, 4, 7],
            2 | 5 | 8 => [2, 5, 8],
            _ => panic!("Invalid block index, {}", block_i),
        };

        let cells_to_check = match cell_i {
            0 | 3 | 6 => [0, 3, 6],
            1 | 4 | 7 => [1, 4, 7],
            2 | 5 | 8 => [2, 5, 8],
            _ => panic!("Invalid cell index: {}", cell_i),
        };

        return self.exists(value, blocks_to_check, cells_to_check);
    }

    pub fn exists_in_block(&self, value: &SudokuValue, cell_position: CellPosition) -> bool {
        let block_i = cell_position.block;
        let cell_not_to_check = cell_position.cell;
        for cell in 0..=8 {
            if cell == cell_not_to_check {
                continue;
            }

            if self.has_value(block_i, cell, value) {
                return true;
            };
        }

        return false;
    }

    pub fn has_value(&self, block_i: usize, cell_i: usize, value: &SudokuValue) -> bool {
        match &self.blocks[block_i][cell_i].value {
            Some(v) => v == value,
            None => false,
        }
    }

    pub fn get(&self, cell_position: CellPosition) -> &SudokuCell {
        &self.blocks[cell_position.block][cell_position.cell]
    }

    pub fn next_up(&mut self, current_cell: CellPosition) -> CellPosition {
        match (current_cell.block, current_cell.cell) {
            (0 | 1 | 2, 0 | 1 | 2) => current_cell,
            (_, 0 | 1 | 2) => CellPosition {
                block: current_cell.block - 3,
                cell: current_cell.cell + 6,
            },
            _ => CellPosition {
                block: current_cell.block,
                cell: current_cell.cell - 3,
            },
        }
    }

    pub fn next_down(&mut self, current_cell: CellPosition) -> CellPosition {
        match (current_cell.block, current_cell.cell) {
            (6 | 7 | 8, 6 | 7 | 8) => current_cell,
            (_, 6 | 7 | 8) => CellPosition {
                block: current_cell.block + 3,
                cell: current_cell.cell - 6,
            },
            _ => CellPosition {
                block: current_cell.block,
                cell: current_cell.cell + 3,
            },
        }
    }

    pub fn next_right(&mut self, current_cell: CellPosition) -> CellPosition {
        match (current_cell.block, current_cell.cell) {
            (2 | 5 | 8, 2 | 5 | 8) => current_cell,
            (_, 2 | 5 | 8) => CellPosition {
                block: current_cell.block + 1,
                cell: current_cell.cell - 2,
            },
            _ => CellPosition {
                block: current_cell.block,
                cell: current_cell.cell + 1,
            },
        }
    }

    pub fn next_left(&mut self, current_cell: CellPosition) -> CellPosition {
        match (current_cell.block, current_cell.cell) {
            (0 | 3 | 6, 0 | 3 | 6) => current_cell,
            (_, 0 | 3 | 6) => CellPosition {
                block: current_cell.block - 1,
                cell: current_cell.cell + 2,
            },
            _ => CellPosition {
                block: current_cell.block,
                cell: current_cell.cell - 1,
            },
        }
    }

    pub fn first_cell(&self) -> CellPosition {
        return CellPosition { block: 0, cell: 0 };
    }

    pub fn next_cell(&self, cell_position: CellPosition) -> Option<CellPosition> {
        if cell_position.block == 8 && cell_position.cell == 8 {
            return None;
        }

        if cell_position.cell == 8 {
            return Some(CellPosition {
                block: cell_position.block + 1,
                cell: 0,
            });
        }

        Some(CellPosition {
            block: cell_position.block,
            cell: cell_position.cell + 1,
        })
    }

    pub fn previous_cell(&self, cell_position: CellPosition) -> Option<CellPosition> {
        if cell_position.block == 0 && cell_position.cell == 0 {
            return None;
        }

        if cell_position.cell == 0 {
            return Some(CellPosition {
                block: cell_position.block - 1,
                cell: 8,
            });
        }

        Some(CellPosition {
            block: cell_position.block,
            cell: cell_position.cell - 1,
        })
    }

    fn exists(
        &self,
        value: &SudokuValue,
        blocks_to_check: [usize; 3],
        cells_to_check: [usize; 3],
    ) -> bool {
        for block in blocks_to_check {
            for cell in cells_to_check {
                if self.has_value(block, cell, value) {
                    return true;
                };
            }
        }

        return false;
    }
}

#[rustfmt::skip]
pub fn block_9by9(block: usize) -> SudokuBlock {
    vec![
        SudokuCell::empty(block, 0), SudokuCell::empty(block, 1), SudokuCell::empty(block, 2),
        SudokuCell::empty(block, 3), SudokuCell::empty(block, 4), SudokuCell::empty(block, 5),
        SudokuCell::empty(block, 6), SudokuCell::empty(block, 7), SudokuCell::empty(block, 8),
    ]
}

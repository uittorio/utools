#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SudokuValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl SudokuValue {
    pub fn number(&self) -> usize {
        match &self {
            SudokuValue::One => 1,
            SudokuValue::Two => 2,
            SudokuValue::Three => 3,
            SudokuValue::Four => 4,
            SudokuValue::Five => 5,
            SudokuValue::Six => 6,
            SudokuValue::Seven => 7,
            SudokuValue::Eight => 8,
            SudokuValue::Nine => 9,
        }
    }
}

pub fn all_values() -> Vec<SudokuValue> {
    vec![
        SudokuValue::One,
        SudokuValue::Two,
        SudokuValue::Three,
        SudokuValue::Four,
        SudokuValue::Five,
        SudokuValue::Six,
        SudokuValue::Seven,
        SudokuValue::Eight,
        SudokuValue::Nine,
    ]
}

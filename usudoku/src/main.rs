mod sudoku_backtracking_generator;
mod sudoku_board;
mod sudoku_generator;
mod sudoku_navigator;
mod sudoku_value;

use iced::{
    Border, Color,
    keyboard::key::Named,
    widget::{Button, Column, Container, button, column, container, row, text},
};

use crate::{
    sudoku_board::{CellPosition, SudokuBlock, SudokuBoard, SudokuCell},
    sudoku_value::SudokuValue,
};

#[derive(Debug, Clone)]
pub enum Message {
    Select(CellPosition),
    Fill(SudokuValue),
    Clear,
    Reset,
    Navigate(Direction),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    sudoku: SudokuBoard,
    selected_cell: Option<CellPosition>,
}

fn main() -> iced::Result {
    iced::application(Game::new, Game::update, Game::view)
        .subscription(Game::subscription)
        .run()
}

impl Game {
    pub fn new() -> Game {
        Game {
            sudoku: sudoku_generator::sudoku_generator(),
            selected_cell: None,
        }
    }

    pub fn reset(&mut self) {
        self.sudoku = sudoku_generator::sudoku_generator();
    }

    pub fn view(&self) -> Column<'_, Message> {
        let grid = self
            .sudoku
            .blocks
            .chunks_exact(3)
            .map(|blocks| {
                row![
                    self.block_container(&blocks[0]),
                    self.block_container(&blocks[1]),
                    self.block_container(&blocks[2]),
                ]
            })
            .fold(Column::new(), |column, row| column.push(row));

        column![grid, button("Reset").on_press(Message::Reset)]
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::listen().filter_map(|event| match event {
            iced::keyboard::Event::KeyPressed { key, .. } => match key {
                iced::keyboard::Key::Named(Named::Backspace) => Some(Message::Clear),
                iced::keyboard::Key::Named(Named::ArrowUp) => {
                    Some(Message::Navigate(Direction::Up))
                }
                iced::keyboard::Key::Named(Named::ArrowDown) => {
                    Some(Message::Navigate(Direction::Down))
                }
                iced::keyboard::Key::Named(Named::ArrowLeft) => {
                    Some(Message::Navigate(Direction::Left))
                }
                iced::keyboard::Key::Named(Named::ArrowRight) => {
                    Some(Message::Navigate(Direction::Right))
                }
                iced::keyboard::Key::Character(c) => match c.as_str() {
                    "1" => Some(Message::Fill(SudokuValue::One)),
                    "2" => Some(Message::Fill(SudokuValue::Two)),
                    "3" => Some(Message::Fill(SudokuValue::Three)),
                    "4" => Some(Message::Fill(SudokuValue::Four)),
                    "5" => Some(Message::Fill(SudokuValue::Five)),
                    "6" => Some(Message::Fill(SudokuValue::Six)),
                    "7" => Some(Message::Fill(SudokuValue::Seven)),
                    "8" => Some(Message::Fill(SudokuValue::Eight)),
                    "9" => Some(Message::Fill(SudokuValue::Nine)),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Select(position) => {
                self.selected_cell = Some(position);
            }
            Message::Clear => match self.selected_cell {
                Some(selected) => {
                    self.sudoku.empty(selected);
                }
                None => {}
            },
            Message::Reset => self.reset(),
            Message::Fill(v) => match self.selected_cell {
                Some(selected) => {
                    self.sudoku.set(selected, v);
                }
                None => {}
            },
            Message::Navigate(direction) => match self.selected_cell {
                Some(_) => match direction {
                    Direction::Up => {
                        self.selected_cell =
                            self.selected_cell.map(|cell| self.sudoku.next_up(cell));
                    }
                    Direction::Down => {
                        self.selected_cell =
                            self.selected_cell.map(|cell| self.sudoku.next_down(cell));
                    }
                    Direction::Left => {
                        self.selected_cell =
                            self.selected_cell.map(|cell| self.sudoku.next_left(cell));
                    }
                    Direction::Right => {
                        self.selected_cell =
                            self.selected_cell.map(|cell| self.sudoku.next_right(cell));
                    }
                },
                None => self.selected_cell = Some(CellPosition { block: 0, cell: 0 }),
            },
        }
    }

    fn block_container<'a>(&self, data: &SudokuBlock) -> Container<'a, Message> {
        container(self.block(&data))
            .padding(3)
            .style(|_theme| container::Style {
                border: Border {
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    width: 2.0,
                    radius: 0.0.into(),
                },
                text_color: None,
                background: None,
                shadow: Default::default(),
                snap: true,
            })
    }

    fn block<'a>(&self, data: &SudokuBlock) -> Column<'a, Message> {
        data.chunks_exact(3)
            .map(|cells| {
                row![
                    self.cell(&cells[0]),
                    self.cell(&cells[1]),
                    self.cell(&cells[2]),
                ]
            })
            .fold(Column::new(), |column, row| column.push(row))
    }

    fn cell<'a>(&self, cell: &SudokuCell) -> Button<'a, Message> {
        let value = match cell.value {
            Some(v) => v.number().to_string(),
            None => String::new(),
        };

        let is_selected = match &self.selected_cell {
            Some(v) => *v == cell.position,
            None => false,
        };

        button(text(value).center().size(20))
            .width(50)
            .height(50)
            .on_press(Message::Select(cell.position))
            .style(move |_theme, _status| button::Style {
                background: None,
                text_color: Color::WHITE,
                snap: true,
                border: Border {
                    color: if is_selected {
                        Color::from_rgb(0.8, 0.8, 0.8)
                    } else {
                        Color::from_rgb(0.6, 0.6, 0.6)
                    },
                    width: if is_selected { 4.0 } else { 1.0 },
                    radius: 8.0.into(),
                },
                shadow: Default::default(),
            })
    }
}

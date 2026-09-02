mod sudoku_board;
mod sudoku_clues;
mod sudoku_game;
mod sudoku_navigator;
mod sudoku_solution;
mod sudoku_solution_generator;
mod sudoku_value;

use std::collections::HashSet;

use iced::{
    Border, Color, Element, Font,
    Length::Fill,
    alignment::{Horizontal, Vertical},
    font,
    keyboard::key::Named,
    widget::{Button, Column, Container, Text, button, column, container, row, text},
};

use crate::{
    sudoku_board::{CellPosition, SudokuBlock, SudokuCell},
    sudoku_game::SudokuGame,
    sudoku_value::SudokuValue,
};

#[derive(Debug, Clone)]
pub enum Message {
    Select(CellPosition),
    Fill(SudokuValue),
    Clear,
    NewGame,
    Navigate(Direction),
    ToggleAnnotation,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    sudoku: SudokuGame,
    selected_cell: Option<CellPosition>,
    errors: HashSet<CellPosition>,
    is_annotation_enabled: bool,
}

fn main() -> iced::Result {
    iced::application(Game::new, Game::update, Game::view)
        .subscription(Game::subscription)
        .run()
}

impl Game {
    pub fn new() -> Game {
        let game = SudokuGame::new();
        let errors = HashSet::new();

        Game {
            sudoku: game,
            selected_cell: None,
            errors,
            is_annotation_enabled: false,
        }
    }

    pub fn new_game(&mut self) {
        self.sudoku = SudokuGame::new();
    }

    pub fn view(&self) -> Column<'_, Message> {
        let grid = self
            .sudoku
            .blocks()
            .chunks_exact(3)
            .map(|blocks| {
                row![
                    self.block_container(&blocks[0]),
                    self.block_container(&blocks[1]),
                    self.block_container(&blocks[2]),
                ]
            })
            .fold(Column::new(), |column, row| column.push(row));

        column![grid, button("New Game").on_press(Message::NewGame)]
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
                    "a" => Some(Message::ToggleAnnotation),
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
                    if self.sudoku.is_clue(selected) == false {
                        match self.sudoku.empty(selected) {
                            Ok(_) => {}
                            Err(_) => {
                                self.errors.insert(selected);
                            }
                        }
                    };
                }
                None => {}
            },
            Message::NewGame => self.new_game(),
            Message::Fill(v) => match self.selected_cell {
                Some(selected) => {
                    if self.sudoku.is_clue(selected) == false {
                        if self.is_annotation_enabled {
                            self.sudoku.annotate(selected, v)
                        } else {
                            self.sudoku.remove_annotations(selected);
                            match self.sudoku.set(selected, v) {
                                Ok(_) => {}
                                Err(_) => {
                                    self.errors.insert(selected);
                                }
                            }
                        }
                    };
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
            Message::ToggleAnnotation => self.is_annotation_enabled = !self.is_annotation_enabled,
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

        let is_error = match cell.value {
            Some(v) => self.sudoku.is_correct(cell.position, v) == false,
            None => false,
        };

        let button_content: Element<'a, Message> = match cell.value {
            Some(_) => text(value)
                .font(Font {
                    family: font::Family::SansSerif,
                    style: if self.is_annotation_enabled {
                        font::Style::Italic
                    } else {
                        font::Style::Normal
                    },
                    ..Font::DEFAULT
                })
                .center()
                .size(20)
                .into(),
            None => {
                let ann = self.annotations(cell);
                ann.into()
            }
        };

        button(button_content)
            .width(50)
            .height(50)
            .on_press(Message::Select(cell.position))
            .style(move |_theme, _status| button::Style {
                background: None,
                text_color: if is_error {
                    Color::from_rgb(1.0, 0.0, 0.0)
                } else {
                    Color::from_rgb(1.0, 1.0, 1.0)
                },
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

    fn annotations<'a>(&self, cell: &SudokuCell) -> Column<'a, Message> {
        column![
            row![
                annotation_or_default(&cell.annotations, SudokuValue::One),
                annotation_or_default(&cell.annotations, SudokuValue::Two),
                annotation_or_default(&cell.annotations, SudokuValue::Three)
            ]
            .width(Fill)
            .height(Fill),
            row![
                annotation_or_default(&cell.annotations, SudokuValue::Four),
                annotation_or_default(&cell.annotations, SudokuValue::Five),
                annotation_or_default(&cell.annotations, SudokuValue::Six)
            ]
            .width(Fill)
            .height(Fill),
            row![
                annotation_or_default(&cell.annotations, SudokuValue::Seven),
                annotation_or_default(&cell.annotations, SudokuValue::Eight),
                annotation_or_default(&cell.annotations, SudokuValue::Nine)
            ]
            .width(Fill)
            .height(Fill),
        ]
        .width(Fill)
        .height(Fill)
    }
}

fn annotation_or_default<'a>(annotations: &HashSet<SudokuValue>, value: SudokuValue) -> Text<'a> {
    let value = match annotations.get(&value) {
        Some(v) => v.number().to_string(),
        None => "".to_string(),
    };

    return text(value)
        .size(14)
        .width(Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);
}

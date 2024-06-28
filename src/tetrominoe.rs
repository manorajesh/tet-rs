use crossterm::style::Color;
use serde::{ Deserialize, Serialize };

use crate::{ bag::Bag, tetlib::EMP };

#[derive(Clone, PartialEq, Debug, Copy, Default, Deserialize, Serialize, Hash)]
pub enum TColor {
    Cyan,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Orange,
    #[default]
    Empty,
}

#[derive(Clone, PartialEq, Debug, Copy, Default, Deserialize, Serialize, Hash)]
pub enum State {
    Landed,
    Active,
    Ghost,
    #[default]
    Empty,
}

#[derive(Clone, PartialEq, Debug, Copy, Default, Deserialize, Serialize, Hash)]
pub struct Tetrominoe {
    pub shape: [[char; 4]; 4],
    pub row: usize,
    pub col: usize,
    pub ptype: char,
    pub color: TColor,
    pub game_state: State,
    pub rotation_state: usize,
    pub number_of_rotations: usize, // **unique** rotations for each piece (1 index)
}

impl Tetrominoe {
    pub fn new(state: Option<State>, color: Option<TColor>) -> Tetrominoe {
        Tetrominoe {
            shape: [[EMP; 4]; 4],
            row: 0,
            col: 0,
            ptype: ' ',
            color: color.unwrap_or(TColor::Empty),
            game_state: state.unwrap_or(State::Empty),
            rotation_state: 0,
            number_of_rotations: 0,
        }
    }

    pub fn set(&mut self, shape: char) -> &mut Self {
        self.ptype = shape;
        let shape = match shape {
            'I' => {
                self.color = TColor::Cyan;
                self.number_of_rotations = 2;
                [
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                ]
            }

            'J' => {
                self.color = TColor::Blue;
                self.number_of_rotations = 4;
                [
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    ['a', 'a', EMP, EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'L' => {
                self.color = TColor::Orange;
                self.number_of_rotations = 4;
                [
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', 'a', EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'O' => {
                self.color = TColor::Yellow;
                self.number_of_rotations = 1;
                [
                    [EMP, EMP, EMP, EMP],
                    [EMP, 'a', 'a', EMP],
                    [EMP, 'a', 'a', EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'Z' => {
                self.color = TColor::Red;
                self.number_of_rotations = 2;
                [
                    [EMP, EMP, EMP, EMP],
                    ['a', 'a', EMP, EMP],
                    [EMP, 'a', 'a', EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'T' => {
                self.color = TColor::Magenta;
                self.number_of_rotations = 4;
                [
                    [EMP, EMP, EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    ['a', 'a', 'a', EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'S' => {
                self.color = TColor::Green;
                self.number_of_rotations = 2;
                [
                    [EMP, EMP, EMP, EMP],
                    [EMP, 'a', 'a', EMP],
                    ['a', 'a', EMP, EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            _ => panic!("Unknown shape: {}", shape),
        };
        self.shape = shape;
        self.rotation_state = 0;
        self
    }

    pub fn set_pos(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn rotate(&mut self) {
        match self.ptype {
            'O' => (),
            'I' | 'J' | 'L' => {
                // transpose or swap rows and columns
                let n = self.shape.len();
                for i in 0..n {
                    for j in i..n {
                        let temp = self.shape[i][j];
                        self.shape[i][j] = self.shape[j][i];
                        self.shape[j][i] = temp;
                    }
                }

                // reverse each row to rotate
                for i in 0..n {
                    self.shape[i].reverse();
                }
            }

            'Z' => {
                if self.rotation_state == 0 {
                    self.shape = [
                        [EMP, EMP, EMP, EMP],
                        [EMP, EMP, 'a', EMP],
                        [EMP, 'a', 'a', EMP],
                        [EMP, 'a', EMP, EMP],
                    ];
                    self.rotation_state = 1;
                } else {
                    self.shape = [
                        [EMP, EMP, EMP, EMP],
                        ['a', 'a', EMP, EMP],
                        [EMP, 'a', 'a', EMP],
                        [EMP, EMP, EMP, EMP],
                    ];
                    self.rotation_state = 0;
                }
            }

            'S' => {
                if self.rotation_state == 0 {
                    self.shape = [
                        [EMP, EMP, EMP, EMP],
                        [EMP, 'a', EMP, EMP],
                        [EMP, 'a', 'a', EMP],
                        [EMP, EMP, 'a', EMP],
                    ];

                    self.rotation_state = 1;
                } else {
                    self.shape = [
                        [EMP, EMP, EMP, EMP],
                        [EMP, 'a', 'a', EMP],
                        ['a', 'a', EMP, EMP],
                        [EMP, EMP, EMP, EMP],
                    ];
                    self.rotation_state = 0;
                }
            }

            'T' => {
                self.shape = match self.rotation_state {
                    0 => {
                        self.rotation_state += 1;
                        [
                            [EMP, 'a', EMP, EMP],
                            [EMP, 'a', 'a', EMP],
                            [EMP, 'a', EMP, EMP],
                            [EMP, EMP, EMP, EMP],
                        ]
                    }

                    1 => {
                        self.rotation_state += 1;
                        [
                            [EMP, EMP, EMP, EMP],
                            ['a', 'a', 'a', EMP],
                            [EMP, 'a', EMP, EMP],
                            [EMP, EMP, EMP, EMP],
                        ]
                    }

                    2 => {
                        self.rotation_state += 1;
                        [
                            [EMP, 'a', EMP, EMP],
                            ['a', 'a', EMP, EMP],
                            [EMP, 'a', EMP, EMP],
                            [EMP, EMP, EMP, EMP],
                        ]
                    }

                    3 => {
                        self.rotation_state = 0;
                        [
                            [EMP, 'a', EMP, EMP],
                            ['a', 'a', 'a', EMP],
                            [EMP, EMP, EMP, EMP],
                            [EMP, EMP, EMP, EMP],
                        ]
                    }

                    _ => panic!("Unknown rotation state: {}", self.rotation_state),
                };
            }

            _ => panic!("Unknown shape: {}", self.ptype),
        }
    }

    pub fn from(ptype: char, state: Option<State>) -> Tetrominoe {
        *Tetrominoe::new(state, None).set(ptype)
    }

    pub fn random(bag: &mut Bag) -> Tetrominoe {
        let piece = bag.draw();
        Tetrominoe::from(piece, None)
    }

    pub fn as_color(&self) -> Color {
        match self.color {
            TColor::Cyan => Color::Cyan,
            TColor::Blue => Color::Blue,
            TColor::Orange =>
                Color::Rgb {
                    r: 255,
                    g: 127,
                    b: 0,
                },
            TColor::Yellow => Color::Yellow,
            TColor::Red => Color::Red,
            TColor::Magenta => Color::Magenta,
            TColor::Green => Color::Green,
            TColor::Empty => Color::Black,
        }
    }
}

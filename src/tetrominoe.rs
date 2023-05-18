use crossterm::style::Color;
use oorandom::Rand32;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::tetlib::EMP;

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
    rotation_state: usize,
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
        }
    }

    pub fn set(&mut self, shape: char) -> &mut Self {
        self.ptype = shape;
        let shape = match shape {
            'I' => {
                self.color = TColor::Cyan;
                [
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                ]
            }

            'J' => {
                self.color = TColor::Blue;
                [
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    ['a', 'a', EMP, EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'L' => {
                self.color = TColor::Orange;
                [
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    [EMP, 'a', 'a', EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'O' => {
                self.color = TColor::Yellow;
                [
                    [EMP, EMP, EMP, EMP],
                    [EMP, 'a', 'a', EMP],
                    [EMP, 'a', 'a', EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'Z' => {
                self.color = TColor::Red;
                [
                    [EMP, EMP, EMP, EMP],
                    ['a', 'a', EMP, EMP],
                    [EMP, 'a', 'a', EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'T' => {
                self.color = TColor::Magenta;
                [
                    [EMP, EMP, EMP, EMP],
                    [EMP, 'a', EMP, EMP],
                    ['a', 'a', 'a', EMP],
                    [EMP, EMP, EMP, EMP],
                ]
            }

            'S' => {
                self.color = TColor::Green;
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
            'I' | 'J' | 'L' | 'T' => {
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

            _ => panic!("Unknown shape: {}", self.ptype),
        }
    }

    pub fn from(ptype: char, state: Option<State>) -> Tetrominoe {
        *Tetrominoe::new(state, None).set(ptype)
    }

    pub fn random() -> Tetrominoe {
        let ptype = match getrandom(7) {
            0 => 'I',
            1 => 'J',
            2 => 'L',
            3 => 'O',
            4 => 'Z',
            5 => 'T',
            6 => 'S',
            _ => panic!("Invalid random number"),
        };
        Tetrominoe::from(ptype, None)
    }

    pub fn as_color(&self) -> Color {
        match self.color {
            TColor::Cyan => Color::Cyan,
            TColor::Blue => Color::Blue,
            TColor::Orange => Color::Rgb {
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

fn getrandom(end: u32) -> u32 {
    let time_from_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    Rand32::new(time_from_epoch).rand_range(0..end)
}

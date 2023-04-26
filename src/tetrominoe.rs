use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};

use crate::tetlib::EMP;

#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct Tetrominoe {
    pub shape: Vec<Vec<char>>,
    pub row: usize,
    pub col: usize,
    pub ptype: char,
    state: usize,
}

impl Tetrominoe {
    pub fn new() -> Tetrominoe {
        Tetrominoe {
            shape: Vec::new(),
            row: 0,
            col: 0,
            ptype: ' ',
            state: 0,
        }
    }

    pub fn set(&mut self, shape: char) -> &mut Self {
        self.ptype = shape;
        let shape = match shape {
            'I' => vec![
                vec![EMP, 'a', EMP, EMP],
                vec![EMP, 'a', EMP, EMP],
                vec![EMP, 'a', EMP, EMP],
                vec![EMP, 'a', EMP, EMP],
            ],

            'J' => vec![
                vec![EMP, 'a', EMP, EMP],
                vec![EMP, 'a', EMP, EMP],
                vec!['a', 'a', EMP, EMP],
                vec![EMP, EMP, EMP, EMP],
            ],

            'L' => vec![
                vec![EMP, 'a', EMP, EMP],
                vec![EMP, 'a', EMP, EMP],
                vec![EMP, 'a', 'a', EMP],
                vec![EMP, EMP, EMP, EMP],
            ],

            'O' => vec![
                vec![EMP, EMP, EMP, EMP],
                vec![EMP, 'a', 'a', EMP],
                vec![EMP, 'a', 'a', EMP],
                vec![EMP, EMP, EMP, EMP],
            ],

            'Z' => vec![
                vec![EMP, EMP, EMP, EMP],
                vec!['a', 'a', EMP, EMP],
                vec![EMP, 'a', 'a', EMP],
                vec![EMP, EMP, EMP, EMP],
            ],

            'T' => vec![
                vec![EMP, EMP, EMP, EMP],
                vec![EMP, 'a', EMP, EMP],
                vec!['a', 'a', 'a', EMP],
                vec![EMP, EMP, EMP, EMP],
            ],

            'S' => vec![
                vec![EMP, EMP, EMP, EMP],
                vec![EMP, 'a', 'a', EMP],
                vec!['a', 'a', EMP, EMP],
                vec![EMP, EMP, EMP, EMP],
            ],

            _ => panic!("Unknown shape: {}", shape),
        };
        self.shape = shape;
        self.state = 0;
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
                if self.state == 0 {
                    self.shape = vec![
                        vec![EMP, EMP, EMP, EMP],
                        vec![EMP, EMP, 'a', EMP],
                        vec![EMP, 'a', 'a', EMP],
                        vec![EMP, 'a', EMP, EMP],
                    ];
                    self.state = 1;
                } else {
                    self.shape = vec![
                        vec![EMP, EMP, EMP, EMP],
                        vec!['a', 'a', EMP, EMP],
                        vec![EMP, 'a', 'a', EMP],
                        vec![EMP, EMP, EMP, EMP],
                    ];
                    self.state = 0;
                }
            }

            'S' => {
                if self.state == 0 {
                    self.shape = vec![
                        vec![EMP, EMP, EMP, EMP],
                        vec![EMP, 'a', EMP, EMP],
                        vec![EMP, 'a', 'a', EMP],
                        vec![EMP, EMP, 'a', EMP],
                    ];

                    self.state = 1;
                } else {
                    self.shape = vec![
                        vec![EMP, EMP, EMP, EMP],
                        vec![EMP, 'a', 'a', EMP],
                        vec!['a', 'a', EMP, EMP],
                        vec![EMP, EMP, EMP, EMP],
                    ];
                    self.state = 0;
                }
            }

            _ => panic!("Unknown shape: {}", self.ptype),
        }
    }

    pub fn from(ptype: char) -> Tetrominoe {
        Tetrominoe::new().set(ptype).clone()
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
        Tetrominoe::from(ptype)
    }
}

fn getrandom(end: usize) -> usize {
    thread_rng().gen_range(0..end)
}

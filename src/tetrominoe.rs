use crate::tetris_lib::EMPTY;

pub struct Tetrominoe {
    pub shape: Vec<Vec<char>>,
    pub row: usize,
    pub col: usize,
}

impl Tetrominoe {
    pub fn new() -> Tetrominoe {
        Tetrominoe {
            shape: Vec::new(),
            row: 0,
            col: 0,
        }
    }

    pub fn set(&mut self, shape: char) {
        let shape = match shape {
            'I' => vec![vec![EMPTY, 'a', EMPTY, EMPTY],
                        vec![EMPTY, 'a', EMPTY, EMPTY],
                        vec![EMPTY, 'a', EMPTY, EMPTY],
                        vec![EMPTY, 'a', EMPTY, EMPTY]],

            'J' => vec![vec![EMPTY, 'a', EMPTY, EMPTY],
                        vec![EMPTY, 'a', EMPTY, EMPTY],
                        vec!['a', 'a', EMPTY, EMPTY],
                        vec![EMPTY, EMPTY, EMPTY, EMPTY]],

            'L' => vec![vec![EMPTY, 'a', EMPTY, EMPTY],
                        vec![EMPTY, 'a', EMPTY, EMPTY],
                        vec![EMPTY, 'a', 'a', EMPTY],
                        vec![EMPTY, EMPTY, EMPTY, EMPTY]],

            'O' => vec![vec![EMPTY, EMPTY, EMPTY, EMPTY],
                        vec![EMPTY, 'a', 'a', EMPTY],
                        vec![EMPTY, 'a', 'a', EMPTY],
                        vec![EMPTY, EMPTY, EMPTY, EMPTY]],

            'S' => vec![vec![EMPTY, EMPTY, EMPTY, EMPTY],
                        vec!['a', 'a', EMPTY, EMPTY],
                        vec![EMPTY, 'a', 'a', EMPTY],
                        vec![EMPTY, EMPTY, EMPTY, EMPTY]],

            'T' => vec![vec![EMPTY, 'a', EMPTY, EMPTY],
                        vec!['a', 'a', 'a', EMPTY],
                        vec![EMPTY, EMPTY, EMPTY, EMPTY],
                        vec![EMPTY, EMPTY, EMPTY, EMPTY]],

            'Z' => vec![vec![EMPTY, EMPTY, EMPTY, EMPTY],
                        vec![EMPTY, 'a', 'a', EMPTY],
                        vec!['a', 'a', EMPTY, EMPTY],
                        vec![EMPTY, EMPTY, EMPTY, EMPTY]],
                        
            _ => panic!("Unknown shape: {}", shape),
        };
        self.shape = shape;
    }

    pub fn set_pos(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn rotate(&mut self) {

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
    
}
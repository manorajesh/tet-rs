use crate::tetris_lib::EMPTY;

pub struct Tetrominoe {
    shape: Vec<Vec<char>>
}

impl Tetrominoe {
    pub fn new(shape: char) -> Tetrominoe {
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
        Tetrominoe { shape }
    }
}
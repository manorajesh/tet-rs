use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Hash)]
pub struct Bag {
    pieces: Vec<char>,
}

impl Bag {
    pub fn new() -> Self {
        let mut pieces = vec!['I', 'J', 'L', 'O', 'S', 'T', 'Z'];
        pieces.shuffle(&mut thread_rng());
        Bag { pieces }
    }

    pub fn draw(&mut self) -> char {
        if self.pieces.is_empty() {
            *self = Bag::new();
        }
        self.pieces.pop().unwrap()
    }
}

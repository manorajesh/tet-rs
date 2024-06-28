use serde::{ Deserialize, Serialize };

use crate::{ gamestate::GameState, gravity_until_new_piece, tetrominoe::State };

#[derive(Serialize, Deserialize, Hash, Debug, Clone)]
pub struct Move {
    pub rotation_state: usize,
    pub x: usize,
}

pub trait AI {
    fn count_completed_lines(&self) -> i32;

    fn count_holes(&self) -> i32;

    fn calculate_col_height(&self, col: usize) -> usize;

    fn calculate_bumpiness(&self) -> i32;

    fn calculate_max_height(&self) -> i32;

    fn evaluate_board(&self) -> i32;

    fn find_best_move(&self) -> Move;

    fn possible_moves(&self) -> Vec<Move>;

    fn make_move(&mut self, mv: &Move);
}

impl AI for GameState {
    fn count_completed_lines(&self) -> i32 {
        let mut completed_lines = 0;

        for row in self.display.iter() {
            if row.iter().all(|&cell| cell.game_state == State::Landed) {
                completed_lines += 1;
            }
        }

        completed_lines
    }

    fn count_holes(&self) -> i32 {
        let mut holes = 0;

        for col in 0..self.display[0].len() {
            let mut found_piece = false;

            for row in 0..self.display.len() {
                if self.display[row][col].game_state == State::Landed {
                    found_piece = true;
                } else if self.display[row][col].game_state == State::Empty && found_piece {
                    holes += 1;
                }
            }
        }

        holes
    }

    fn calculate_col_height(&self, col: usize) -> usize {
        for row in 0..self.display.len() {
            if self.display[row][col].game_state == State::Landed {
                return self.display.len() - row;
            }
        }

        0
    }

    fn calculate_bumpiness(&self) -> i32 {
        let mut bumpiness = 0;

        for col in 0..self.display[0].len() - 1 {
            let col_height = self.calculate_col_height(col);
            let next_col_height = self.calculate_col_height(col + 1);

            bumpiness += ((col_height as i32) - (next_col_height as i32)).abs();
        }

        bumpiness
    }

    fn calculate_max_height(&self) -> i32 {
        let mut max_height = 0;

        for col in 0..self.display[0].len() {
            let col_height = self.calculate_col_height(col);

            if col_height > max_height {
                max_height = col_height;
            }
        }

        max_height as i32
    }

    fn evaluate_board(&self) -> i32 {
        let completed_lines = self.count_completed_lines();
        let holes = self.count_holes();
        let bumpiness = self.calculate_bumpiness();
        let max_height = self.calculate_max_height();

        // Weighted sum of features
        completed_lines * 100 - holes * 50 - bumpiness * 10 - max_height * 1
    }

    fn find_best_move(&self) -> Move {
        let mut best_move = None;
        let mut best_score = i32::MIN;

        for possible_move in self.possible_moves() {
            let mut test_game = self.clone();
            test_game.make_move(&possible_move);
            let score = test_game.evaluate_board();

            if score > best_score {
                best_score = score;
                best_move = Some(possible_move);
            }
        }

        best_move.unwrap()
    }

    fn possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for rotation_state in 0..self.active_piece.number_of_rotations {
            for x in 0..self.display[0].len() {
                moves.push(Move {
                    rotation_state,
                    x,
                });
            }
        }

        moves
    }

    fn make_move(&mut self, mv: &Move) {
        self.active_piece.col = mv.x;
        self.active_piece.rotation_state = mv.rotation_state;
        gravity_until_new_piece(self);
    }
}

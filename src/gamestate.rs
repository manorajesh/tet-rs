use crate::tetlib::new_piece;
use crate::{tetrominoe::Tetrominoe, gamescore::GameScore, tetlib::init};
use std::fs::OpenOptions;
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub display: Vec<Vec<char>>,
    pub active_piece: Tetrominoe,
    pub gamescore: GameScore,
    pub hold_piece: Option<Tetrominoe>,
    pub next_piece: Tetrominoe,
    pub counter: usize,
    pub is_game_over: bool,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let mut gs = GameState {
            display: init(width, height),
            active_piece: Tetrominoe::new(),
            gamescore: GameScore::new(),
            hold_piece: None,
            next_piece: Tetrominoe::random(),
            counter: 0,
            is_game_over: false,
        };
        init(width, height);
        new_piece(&mut gs.display, &mut gs.active_piece, None, &mut gs.next_piece);
        gs
    }

    pub fn serial(&mut self, path: String) {
        let path = if confirmation("Save game?") {
            path
        } else {
            return;
        };

        if Path::new(&path).exists() {
            if !confirmation("Overwrite save file?") {
                return;
            }
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();

        let serialized_data = serialize(&self).unwrap();
        file.write_all(&serialized_data).unwrap();
    }

    pub fn deserial(path: String, width: usize, height: usize) -> Self {
        if !Path::new(&path).exists() {
            return GameState::new(width, height);
        }

        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .unwrap();

        let mut serialized_data = Vec::new();
        file.read_to_end(&mut serialized_data).unwrap();

        let mut game: GameState = deserialize(&serialized_data).unwrap();
        game.gamescore.reset_timer();
        init(width, height);
        game
    }
}

fn confirmation(prompt: &str) -> bool {
    loop {
        println!("{} (y/n): ", prompt);
        let stdin = std::io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        println!();
        match buf.trim() {
            "y" => return true,
            "n" => return false,
            _ => continue,
        }
    }
}
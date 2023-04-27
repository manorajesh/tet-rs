use crate::tetlib::new_piece;
use crate::{gamescore::GameScore, tetlib::init, tetrominoe::Tetrominoe};
use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::{Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct GameWrapper {
    game: GameState,
    hash: u64,
}

impl GameWrapper {
    fn verify(&self) -> bool {
        let mut hasher = DefaultHasher::new();
        self.game.hash(&mut hasher);
        let hash = hasher.finish();
        hash == self.hash
    }
}

#[derive(Serialize, Deserialize, Clone, Hash)]
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
        new_piece(
            &mut gs.display,
            &mut gs.active_piece,
            None,
            &mut gs.next_piece,
        );
        gs
    }

    pub fn serial(&mut self) {
        if !confirmation("Save game?") {
            return;
        }

        let path = if confirmation("Use default save file?") {
            String::from("save.tetris")
        } else {
            handle_input!("Enter path to save file: ")
        };

        if Path::new(&path).exists() {
            if !confirmation("Overwrite save file?") {
                return;
            }
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();

        self.gamescore.stop_timer();
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();

        let game_wrapper = GameWrapper {
            game: self.clone(),
            hash,
        };

        if !game_wrapper.verify() {
            println!("Hash verification failed. Aborting save.");
            return;
        }

        let serialized_data = serialize(&game_wrapper).expect("Failed to serialize game.");
        file.write_all(&serialized_data).unwrap();
    }

    pub fn deserial(path: String, width: usize, height: usize) -> Self {
        if !Path::new(&path).exists() {
            return GameState::new(width, height);
        }

        let mut file = OpenOptions::new().read(true).open(path).unwrap();

        let mut serialized_data = Vec::new();
        file.read_to_end(&mut serialized_data).unwrap();

        let game_wrapper: GameWrapper =
            deserialize(&serialized_data).expect("Failed to deserialize game.");

        if !game_wrapper.verify() {
            println!("Save file is corrupted. Starting new game.");
            sleep(Duration::from_secs(2));
            return GameState::new(width, height);
        }

        let mut game = game_wrapper.game;
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

#[macro_export]
macro_rules! handle_input {
    ($x:expr) => {{
        print!("{}", $x);
        std::io::stdout().flush().unwrap();
        let stdin = std::io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }};
}
pub(crate) use handle_input;

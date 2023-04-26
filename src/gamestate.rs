use crate::tetlib::{get_input, new_piece};
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
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        GameState {
            display: init(width, height),
            active_piece: Tetrominoe::new(),
            gamescore: GameScore::new(),
            hold_piece: None,
            next_piece: Tetrominoe::random(),
            counter: 0,
        }
    }

    pub fn serial(&self, path: Option<String>) {
        let path = if Path::new(&path.clone().unwrap_or("save.trs".to_string()).clone()).exists() {
            print!("File already exists, overwrite? (y/n): ");
            if get_input() == 'y' {
                "save.trs".to_string()
            } else {
                return;
            }
        } else {
            path.unwrap_or("save.trs".to_string())
        };

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();

        let serialized_data = serialize(&self).unwrap();
        file.write_all(&serialized_data).unwrap();
    }

    pub fn deserial(path: Option<String>, width: usize, height: usize) -> Self {
        if !Path::new(&path.clone().unwrap_or("save.trs".to_string())).exists() {
            println!("No save file found, starting new game");
            let mut gs = GameState::new(width, height);
            new_piece(&mut gs.display, &mut gs.active_piece, None, &mut gs.next_piece);
            return gs;
        } else {
            println!("Loading save file");
        }

        let mut file = OpenOptions::new()
            .read(true)
            .open(path.unwrap_or("save.trs".to_string()))
            .unwrap();

        let mut serialized_data = Vec::new();
        file.read_to_end(&mut serialized_data).unwrap();

        deserialize(&serialized_data).unwrap()
    }
}
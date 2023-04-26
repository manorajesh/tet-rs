use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GameScore {
    pub score: usize,
    pub level: usize,
    pub time: SystemTime,
}

impl GameScore {
    pub fn new() -> GameScore {
        GameScore {
            score: 0,
            level: 0,
            time: SystemTime::now(),
        }
    }

    pub fn get_time(&self) -> usize {
        let now = SystemTime::now();
        let duration = now.duration_since(self.time).unwrap();
        duration.as_secs() as usize
    }
}

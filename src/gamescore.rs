use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Hash, Clone)]
pub struct GameScore {
    pub score: usize,
    pub level: usize,
    pub elapsed_time: Duration,
    #[serde(skip)]
    last_update: Option<Instant>,
}

impl GameScore {
    pub fn new() -> GameScore {
        GameScore {
            score: 0,
            level: 0,
            elapsed_time: Duration::new(0, 0),
            last_update: Some(Instant::now()),
        }
    }

    pub fn update(&mut self) {
        if let Some(last_update) = self.last_update {
            let now = Instant::now();
            let duration_since_last_update = now.duration_since(last_update);

            if duration_since_last_update >= Duration::from_secs(1) {
                self.elapsed_time += Duration::from_secs(1);
                self.last_update = Some(now);
            }
        }
    }

    pub fn reset_timer(&mut self) {
        self.last_update = Some(Instant::now());
    }

    pub fn get_time(&self) -> usize {
        self.elapsed_time.as_secs() as usize
    }

    pub fn stop_timer(&mut self) {
        self.last_update = None;
    }
}

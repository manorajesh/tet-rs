pub struct GameScore {
    pub score: usize,
    pub level: usize
}

impl GameScore {
    pub fn new() -> GameScore {
        GameScore { score: 0, level: 0 }
    }
}
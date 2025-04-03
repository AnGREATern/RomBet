use super::Winner;

const POINTS_PER_WIN: i32 = 3;

pub struct PastResults {
    pub wins: u32,
    pub draws: u32,
    pub loses: u32,
}

impl PastResults {
    pub fn new() -> Self {
        let wins = 0;
        let draws = 0;
        let loses = 0;
        Self { wins, draws, loses }
    }

    pub fn pts_diff(&self) -> i32 {
        POINTS_PER_WIN * (self.wins as i32 - self.loses as i32)
    }

    pub fn add_result(&mut self, winner: Winner) {
        match winner {
            Winner::W1 => self.wins += 1,
            Winner::X => self.draws += 1,
            Winner::W2 => self.loses += 1,
        }
    }
}

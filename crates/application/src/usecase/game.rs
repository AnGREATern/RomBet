use anyhow::Result;

use domain::entity::{Game, GameStat, Simulation};

pub trait RandomizeRound {
    fn randomize_game(&mut self, game: &Game) -> Result<GameStat>;

    fn randomize_round(&mut self, simulation: &Simulation) -> Result<Vec<GameStat>>;
}

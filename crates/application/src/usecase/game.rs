use anyhow::Result;

use domain::entity::{Game, Simulation};

pub trait RandomizeRound {
    fn randomize_game(&mut self, game: &Game) -> Result<()>;

    fn randomize_round(&mut self, simulation: &Simulation) -> Result<()>;
}

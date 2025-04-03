use anyhow::Result;

use domain::entity::{Game, Simulation};

pub trait RandomizeRound {
    fn randomize_game(&self, game: &Game) -> Result<()>;

    fn randomize_round(&self, simulation: &Simulation) -> Result<()>;
}

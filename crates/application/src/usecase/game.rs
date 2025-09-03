use anyhow::Result;

use crate::service::DisplayedGameStat;
use domain::entity::{Game, Simulation};

pub trait RandomizeRound {
    fn randomize_game(&self, game: &Game) -> Result<DisplayedGameStat>;

    fn randomize_round(&self, simulation: &Simulation) -> Result<Vec<DisplayedGameStat>>;
}

use anyhow::Result;

use domain::entity::{Game, Simulation};
use crate::service::DisplayedGameStat;

pub trait RandomizeRound {
    fn randomize_game(&self, game: &Game) -> Result<DisplayedGameStat>;

    fn randomize_round(&self, simulation: &Simulation) -> Result<Vec<DisplayedGameStat>>;
}

use anyhow::Result;

use crate::service::DisplayedGameStat;
use domain::entity::{Game, Simulation};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait RandomizeRound {
    fn randomize_game(&self, game: &Game) -> Result<DisplayedGameStat>;

    fn randomize_round(&self, simulation: &Simulation) -> Result<Vec<DisplayedGameStat>>;
}

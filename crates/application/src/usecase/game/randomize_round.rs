use anyhow::Result;

use domain::{entity::Game, value_object::Id};

pub trait RandomizeRound {
    fn randomize_game(&self, game_id: Id<Game>) -> Result<()>;

    fn randomize_round(&self, round: u32) -> Result<()>;
}
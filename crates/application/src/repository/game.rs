use anyhow::Result;

use domain::{
    entity::{Game, Team},
    value_object::Id,
};

pub trait IGameRepo {
    fn new() -> Self;

    fn add(&self, game: Game) -> Result<Id<Game>>;

    fn game_by_id(&self, game_id: Id<Game>) -> Result<Game>;

    fn games_id_by_team_id(&self, team_id: Id<Team>, cnt: u8) -> Result<Vec<Id<Game>>>;

    fn games_id_by_round(&self, round: u32) -> Result<Vec<Id<Game>>>;

    fn h2hs_id_by_team_id(
        &self,
        home_team_id: Id<Team>,
        guest_team_id: Id<Team>,
        cnt: u8,
    ) -> Result<Vec<Id<Game>>>;

    fn next_id(&self) -> Id<Game>;

    fn reset(&self);
}

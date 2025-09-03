use anyhow::Result;

use domain::{
    entity::{Game, Simulation, Team},
    value_object::Id,
};

pub trait IGameRepo {
    fn add(&self, game: Game) -> Result<()>;

    fn game_by_id(&self, game_id: Id<Game>) -> Result<Game>;

    fn games_id_by_team_id(
        &self,
        team_id: Id<Team>,
        simulation_id: Id<Simulation>,
        cnt: u8,
    ) -> Result<Vec<(Id<Game>, bool)>>;

    fn games_id_by_round(
        &self,
        round: u32,
        simulation_id: Id<Simulation>,
    ) -> Result<Vec<Id<Game>>>;

    fn h2hs_id_by_team_id(
        &self,
        home_team_id: Id<Team>,
        guest_team_id: Id<Team>,
        simulation_id: Id<Simulation>,
        cnt: u8,
    ) -> Result<Vec<(Id<Game>, bool)>>;

    fn next_id(&self) -> Id<Game>;
}

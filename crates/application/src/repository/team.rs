use anyhow::Result;

use domain::{entity::Team, value_object::Id};

pub trait ITeamRepo {
    fn new() -> Self;

    fn all_teams_id(&mut self) -> Vec<Id<Team>>;

    fn team_by_id(&mut self, id: Id<Team>) -> Result<Team>;
}

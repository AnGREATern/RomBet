use anyhow::Result;

use domain::{entity::Team, value_object::Id};

pub trait ITeamRepo {
    fn new() -> Self;

    fn team_by_id(&self, team_id: Id<Team>) -> Result<Team>;

    fn all_teams_id(&self) -> Vec<Id<Team>>;
}
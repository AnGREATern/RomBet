use anyhow::Result;

use domain::{entity::Team, value_object::Id};

pub trait ITeamRepo {
    fn all_teams_id(&self) -> Vec<Id<Team>>;

    fn team_by_id(&self, id: Id<Team>) -> Result<Team>;
}

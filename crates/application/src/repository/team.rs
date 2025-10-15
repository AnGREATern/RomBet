use anyhow::Result;

use domain::{entity::Team, value_object::Id};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait ITeamRepo {
    fn all_teams_id(&self) -> Vec<Id<Team>>;

    fn all_teams(&self) -> Vec<Team>;

    fn add(&self, team: Team) -> Result<()>;

    fn update(&self, team: Team) -> Result<()>;

    fn delete(&self, id: Id<Team>) -> Result<()>;

    fn team_by_id(&self, id: Id<Team>) -> Result<Team>;

    fn next_id(&self) -> Id<Team>;
}

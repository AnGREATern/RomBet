use anyhow::Result;
use domain::entity::Team;

use crate::{repository::ITeamRepo, usecase::CreateTeam};
pub struct TeamService<T: ITeamRepo> {
    team_repo: T,
}

impl<T: ITeamRepo> CreateTeam for TeamService<T> {
    fn add_team<S: ToString>(&self, name: S) -> Result<()> {
        let id = self.team_repo.next_id();
        let team = Team::new(id, name.to_string());

        self.team_repo.add(team)
    }
}

impl<T: ITeamRepo> TeamService<T> {
    pub fn new(team_repo: T) -> Self {
        Self { team_repo }
    }
}

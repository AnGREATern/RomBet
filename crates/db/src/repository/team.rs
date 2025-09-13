use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

use crate::DBPool;
use crate::models::TeamPostrgres;
use application::repository::ITeamRepo;
use domain::{entity::Team, value_object::Id};

impl From<TeamPostrgres> for Team {
    fn from(t: TeamPostrgres) -> Self {
        Self::new(t.id.into(), t.name)
    }
}

pub struct TeamRepo {
    pool: DBPool,
}

impl TeamRepo {
    pub fn new(pool: DBPool) -> Self {
        Self { pool }
    }
}

impl ITeamRepo for TeamRepo {
    fn all_teams_id(&self) -> Vec<Id<Team>> {
        use crate::schema::team::dsl::*;

        let mut connection = self.pool.get().unwrap();
        team.select(id)
            .load(&mut connection)
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|e: Uuid| e.into())
            .collect()
    }

    fn team_by_id(&self, q_id: Id<Team>) -> Result<Team> {
        use crate::schema::team::dsl::*;

        let mut connection = self.pool.get()?;
        let t = team
            .filter(id.eq(&q_id.value()))
            .select(TeamPostrgres::as_select())
            .first(&mut connection)?
            .into();

        Ok(t)
    }
}

use crate::{establish_connection, models::TeamPostrgres};
use application::repository::ITeamRepo;
use domain::{entity::Team, value_object::Id};

use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

pub struct TeamRepo {
    connection: PgConnection,
}

impl From<TeamPostrgres> for Team {
    fn from(t: TeamPostrgres) -> Self {
        Self::new(t.id.into(), t.name)
    }
}

impl ITeamRepo for TeamRepo {
    fn new() -> Self {
        let connection = establish_connection();
        Self { connection }
    }

    fn all_teams_id(&mut self) -> Vec<Id<Team>> {
        use crate::schema::team::dsl::*;

        team.select(id)
            .load(&mut self.connection)
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|e: Uuid| e.into())
            .collect()
    }

    fn team_by_id(&mut self, q_id: Id<Team>) -> Result<Team> {
        use crate::schema::team::dsl::*;

        let t = team
            .filter(id.eq(&q_id.value()))
            .select(TeamPostrgres::as_select())
            .first(&mut self.connection)?
            .into();

        Ok(t)
    }
}

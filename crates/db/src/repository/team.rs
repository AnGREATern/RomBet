use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

use crate::DBPool;
use crate::models::TeamPostgres;
use application::repository::ITeamRepo;
use domain::{entity::Team, value_object::Id};

impl From<TeamPostgres> for Team {
    fn from(t: TeamPostgres) -> Self {
        Self::new(t.id.into(), t.name)
    }
}

impl From<Team> for TeamPostgres {
    fn from(t: Team) -> Self {
        Self {
            id: t.id().value(),
            name: t.name().to_string(),
        }
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

    fn all_teams(&self) -> Vec<Team> {
        use crate::schema::team::dsl::*;

        let mut connection = self.pool.get().unwrap();
        team.select(TeamPostgres::as_select())
            .load(&mut connection)
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|t: TeamPostgres| t.into())
            .collect()
    }

    fn add(&self, team: Team) -> Result<()> {
        use crate::schema::team as STeam;

        let mut connection = self.pool.get()?;
        let team = TeamPostgres::from(team);
        diesel::insert_into(STeam::table)
            .values(&team)
            .execute(&mut connection)?;

        Ok(())
    }

    fn update(&self, team: Team) -> Result<()> {
        use crate::schema::team::{
            self,
            dsl::{id, name},
        };

        let mut connection = self.pool.get()?;
        diesel::update(team::table)
            .filter(id.eq(&team.id().value()))
            .set(name.eq(team.name()))
            .execute(&mut connection)?;

        Ok(())
    }

    fn delete(&self, team_id: Id<Team>) -> Result<()> {
        use crate::schema::team::{dsl::id, table};

        let mut connection = self.pool.get()?;
        diesel::delete(table)
            .filter(id.eq(team_id.value()))
            .execute(&mut connection)?;

        Ok(())
    }

    fn team_by_id(&self, q_id: Id<Team>) -> Result<Team> {
        use crate::schema::team::dsl::*;

        let mut connection = self.pool.get()?;
        let t = team
            .filter(id.eq(&q_id.value()))
            .select(TeamPostgres::as_select())
            .first(&mut connection)?
            .into();

        Ok(t)
    }

    fn next_id(&self) -> Id<Team> {
        Id::new()
    }
}

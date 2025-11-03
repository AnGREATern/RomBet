use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

use crate::DBPool;
use crate::models::TeamSqlite;
use application::repository::ITeamRepo;
use domain::{entity::Team, value_object::Id};

impl From<TeamSqlite> for Team {
    fn from(t: TeamSqlite) -> Self {
        Self::new(Uuid::parse_str(&t.id).unwrap().into(), t.name)
    }
}

impl From<Team> for TeamSqlite {
    fn from(t: Team) -> Self {
        Self {
            id: t.id().value().to_string(),
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
        use crate::schema::Team::dsl::*;

        let mut connection = self.pool.get().unwrap();
        Team.select(id)
            .load(&mut connection)
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|e: String| Uuid::parse_str(&e).unwrap().into())
            .collect()
    }

    fn all_teams(&self) -> Vec<Team> {
        use crate::schema::Team::dsl::*;

        let mut connection = self.pool.get().unwrap();
        Team.select(TeamSqlite::as_select())
            .load(&mut connection)
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|t: TeamSqlite| t.into())
            .collect()
    }

    fn add(&self, team: Team) -> Result<()> {
        use crate::schema::Team as STeam;

        let mut connection = self.pool.get()?;
        let team = TeamSqlite::from(team);
        diesel::insert_into(STeam::table)
            .values(&team)
            .execute(&mut connection)?;

        Ok(())
    }

    fn update(&self, team: Team) -> Result<()> {
        use crate::schema::Team::{
            self,
            dsl::{id, name},
        };

        let mut connection = self.pool.get()?;
        diesel::update(Team::table)
            .filter(id.eq(&team.id().value().to_string()))
            .set(name.eq(team.name()))
            .execute(&mut connection)?;

        Ok(())
    }

    fn delete(&self, team_id: Id<Team>) -> Result<()> {
        use crate::schema::Team::{dsl::id, table};

        let mut connection = self.pool.get()?;
        diesel::delete(table)
            .filter(id.eq(team_id.value().to_string()))
            .execute(&mut connection)?;

        Ok(())
    }

    fn team_by_id(&self, q_id: Id<Team>) -> Result<Team> {
        use crate::schema::Team::dsl::*;

        let mut connection = self.pool.get()?;
        let t = Team
            .filter(id.eq(&q_id.value().to_string()))
            .select(TeamSqlite::as_select())
            .first(&mut connection)?
            .into();

        Ok(t)
    }

    fn next_id(&self) -> Id<Team> {
        Id::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::TeamRepo;
    use crate::repository::common::pool;
    use application::repository::ITeamRepo;
    use diesel::SqliteConnection;
    use rstest::*;

    #[rstest]
    fn select_all_teams(
        pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>,
    ) {
        let repo = TeamRepo::new(pool.clone());

        let ids = repo.all_teams_id();

        assert_eq!(15, ids.len());
    }
}

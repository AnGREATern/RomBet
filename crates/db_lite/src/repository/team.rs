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
}

#[cfg(test)]
mod tests {
    use crate::init_pool;
    use crate::repository::TeamRepo;
    use crate::repository::common::run_migrations;
    use application::repository::ITeamRepo;

    #[test]
    fn select_all_teams() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let repo = TeamRepo::new(pool.clone());

        let ids = repo.all_teams_id();

        assert_eq!(15, ids.len());
    }
}

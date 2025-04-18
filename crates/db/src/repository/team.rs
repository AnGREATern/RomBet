use crate::establish_connection;
use application::repository::ITeamRepo;
use domain::{entity::Team, value_object::Id};

use diesel::prelude::*;
use uuid::Uuid;

pub struct TeamRepo {
    connection: PgConnection,
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
}

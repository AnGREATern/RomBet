use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

use crate::DBPool;
use crate::models::GameSqlite;
use application::repository::IGameRepo;
use domain::{
    entity::{Game, Simulation, Team},
    value_object::Id,
};

impl From<Game> for GameSqlite {
    fn from(g: Game) -> Self {
        Self {
            id: g.id().value().to_string(),
            simulation_id: g.simulation_id().value().to_string(),
            home_team_id: g.home_team_id().value().to_string(),
            guest_team_id: g.guest_team_id().value().to_string(),
            round: g.round() as i64,
        }
    }
}

impl From<GameSqlite> for Game {
    fn from(g: GameSqlite) -> Self {
        Game::new(
            Uuid::parse_str(&g.id).unwrap().into(),
            Uuid::parse_str(&g.simulation_id).unwrap().into(),
            Uuid::parse_str(&g.home_team_id).unwrap().into(),
            Uuid::parse_str(&g.guest_team_id).unwrap().into(),
            g.round as u32,
        )
    }
}

pub struct GameRepo {
    pool: DBPool,
}

impl GameRepo {
    pub fn new(pool: DBPool) -> Self {
        Self { pool }
    }
}

impl IGameRepo for GameRepo {
    fn add(&self, game: Game) -> Result<()> {
        use crate::schema::Game;

        let mut connection = self.pool.get()?;
        let game = GameSqlite::from(game);
        diesel::insert_into(Game::table)
            .values(&game)
            .execute(&mut connection)?;

        Ok(())
    }

    fn game_by_id(&self, game_id: Id<Game>) -> Result<Game> {
        use crate::schema::Game::dsl::*;

        let mut connection = self.pool.get()?;
        let rec = Game
            .filter(id.eq(game_id.value().to_string()))
            .select(GameSqlite::as_select())
            .first(&mut connection)?;

        Ok(rec.into())
    }

    fn games_id_by_round(&self, rnd: u32, sim_id: Id<Simulation>) -> Result<Vec<Id<Game>>> {
        use crate::schema::Game::dsl::*;

        let mut connection = self.pool.get()?;
        let recs = Game
            .filter(simulation_id.eq(sim_id.value().to_string()))
            .filter(round.eq(rnd as i64))
            .select(id)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: String| Uuid::parse_str(&elem).unwrap().into())
            .collect();

        Ok(recs)
    }

    fn games_id_by_team_id(
        &self,
        team_id: Id<Team>,
        sim_id: Id<Simulation>,
        cnt: u8,
    ) -> Result<Vec<(Id<Game>, bool)>> {
        use crate::schema::Game::dsl::*;

        let mut connection = self.pool.get()?;
        let mut games = Game
            .filter(simulation_id.eq(sim_id.value().to_string()))
            .filter(home_team_id.eq(team_id.value().to_string()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: String| (Uuid::parse_str(&elem).unwrap().into(), true))
            .collect::<Vec<(Id<domain::entity::Game>, bool)>>();
        let mut guest_games = Game
            .filter(simulation_id.eq(sim_id.value().to_string()))
            .filter(guest_team_id.eq(team_id.value().to_string()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: String| (Uuid::parse_str(&elem).unwrap().into(), false))
            .collect();

        games.append(&mut guest_games);

        Ok(games)
    }

    fn h2hs_id_by_team_id(
        &self,
        ht_id: Id<Team>,
        gt_id: Id<Team>,
        sim_id: Id<Simulation>,
        cnt: u8,
    ) -> Result<Vec<(Id<Game>, bool)>> {
        use crate::schema::Game::dsl::*;

        let mut connection = self.pool.get()?;
        let mut games = Game
            .filter(simulation_id.eq(sim_id.value().to_string()))
            .filter(home_team_id.eq(ht_id.value().to_string()))
            .filter(guest_team_id.eq(gt_id.value().to_string()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: String| (Uuid::parse_str(&elem).unwrap().into(), true))
            .collect::<Vec<(Id<domain::entity::Game>, bool)>>();
        let mut inverse_games = Game
            .filter(simulation_id.eq(sim_id.value().to_string()))
            .filter(home_team_id.eq(gt_id.value().to_string()))
            .filter(guest_team_id.eq(ht_id.value().to_string()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: String| (Uuid::parse_str(&elem).unwrap().into(), false))
            .collect();

        games.append(&mut inverse_games);

        Ok(games)
    }

    fn next_id(&self) -> Id<Game> {
        Id::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::init_pool;
    use crate::repository::common::{GameFactory, SimulationBuilder, run_migrations};
    use crate::repository::{GameRepo, SimulationRepo};
    use application::repository::{IGameRepo, ISimulationRepo};

    #[test]
    fn game_by_id_found() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let game_repo = GameRepo::new(pool.clone());
        let sim_repo = SimulationRepo::new(pool.clone());
        let simulation = SimulationBuilder::new().build();
        sim_repo.add(simulation).unwrap();
        let game = GameFactory::create_spa_zen_game(simulation.id());
        game_repo.add(game).unwrap();

        let rec = game_repo.game_by_id(game.id());

        assert!(rec.is_ok());
    }

    #[test]
    fn game_by_id_not_found() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let game_repo = GameRepo::new(pool.clone());
        let sim_repo = SimulationRepo::new(pool.clone());
        let simulation = SimulationBuilder::new().build();
        sim_repo.add(simulation).unwrap();
        let game = GameFactory::create_spa_zen_game(simulation.id());
        game_repo.add(game).unwrap();

        let rec = game_repo.game_by_id(simulation.id().value().into());

        assert!(rec.is_err());
    }
}

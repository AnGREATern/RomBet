use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

use crate::DBPool;
use crate::models::GamePostgres;
use application::repository::IGameRepo;
use domain::{
    entity::{Game, Simulation, Team},
    value_object::Id,
};

impl From<Game> for GamePostgres {
    fn from(g: Game) -> Self {
        Self {
            id: g.id().value(),
            simulation_id: g.simulation_id().value(),
            home_team_id: g.home_team_id().value(),
            guest_team_id: g.guest_team_id().value(),
            round: g.round() as i64,
        }
    }
}

impl From<GamePostgres> for Game {
    fn from(g: GamePostgres) -> Self {
        Game::new(
            g.id.into(),
            g.simulation_id.into(),
            g.home_team_id.into(),
            g.guest_team_id.into(),
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
        use crate::schema::game;

        let mut connection = self.pool.get()?;
        let game = GamePostgres::from(game);
        diesel::insert_into(game::table)
            .values(&game)
            .execute(&mut connection)?;

        Ok(())
    }

    fn game_by_id(&self, game_id: Id<Game>) -> Result<Game> {
        use crate::schema::game::dsl::*;

        let mut connection = self.pool.get()?;
        let rec = game
            .filter(id.eq(game_id.value()))
            .select(GamePostgres::as_select())
            .first(&mut connection)?;

        Ok(rec.into())
    }

    fn games_id_by_round(&self, rnd: u32, sim_id: Id<Simulation>) -> Result<Vec<Id<Game>>> {
        use crate::schema::game::dsl::*;

        let mut connection = self.pool.get()?;
        let recs = game
            .filter(simulation_id.eq(sim_id.value()))
            .filter(round.eq(rnd as i64))
            .select(id)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: Uuid| elem.into())
            .collect();

        Ok(recs)
    }

    fn games_id_by_team_id(
        &self,
        team_id: Id<Team>,
        sim_id: Id<Simulation>,
        cnt: u8,
    ) -> Result<Vec<(Id<Game>, bool)>> {
        use crate::schema::game::dsl::*;

        let mut connection = self.pool.get()?;
        let mut games = game
            .filter(simulation_id.eq(sim_id.value()))
            .filter(home_team_id.eq(team_id.value()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: Uuid| (elem.into(), true))
            .collect::<Vec<(Id<Game>, bool)>>();
        let mut guest_games = game
            .filter(simulation_id.eq(sim_id.value()))
            .filter(guest_team_id.eq(team_id.value()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: Uuid| (elem.into(), false))
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
        use crate::schema::game::dsl::*;

        let mut connection = self.pool.get()?;
        let mut games = game
            .filter(simulation_id.eq(sim_id.value()))
            .filter(home_team_id.eq(ht_id.value()))
            .filter(guest_team_id.eq(gt_id.value()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: Uuid| (elem.into(), true))
            .collect::<Vec<(Id<Game>, bool)>>();
        let mut inverse_games = game
            .filter(simulation_id.eq(sim_id.value()))
            .filter(home_team_id.eq(gt_id.value()))
            .filter(guest_team_id.eq(ht_id.value()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut connection)?
            .into_iter()
            .map(|elem: Uuid| (elem.into(), false))
            .collect();

        games.append(&mut inverse_games);

        Ok(games)
    }

    fn next_id(&self) -> Id<Game> {
        Id::new()
    }
}

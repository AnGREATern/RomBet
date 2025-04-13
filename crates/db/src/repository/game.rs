use crate::{establish_connection, models::GamePostrgres};
use application::repository::IGameRepo;
use domain::{
    entity::{Game, Simulation, Team},
    value_object::Id,
};

use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

impl From<Game> for GamePostrgres {
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

impl From<GamePostrgres> for Game {
    fn from(g: GamePostrgres) -> Self {
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
    connection: PgConnection,
}

impl IGameRepo for GameRepo {
    fn new() -> Self {
        let connection = establish_connection();
        Self { connection }
    }

    fn add(&mut self, game: Game) -> Result<()> {
        use crate::schema::game;

        let game = GamePostrgres::from(game);
        diesel::insert_into(game::table)
            .values(&game)
            .execute(&mut self.connection)?;

        Ok(())
    }

    fn game_by_id(&mut self, game_id: Id<Game>) -> Result<Game> {
        use crate::schema::game::dsl::*;

        let rec = game
            .filter(id.eq(game_id.value()))
            .select(GamePostrgres::as_select())
            .first(&mut self.connection)?;

        Ok(rec.into())
    }

    fn games_id_by_round(&mut self, rnd: u32, sim_id: Id<Simulation>) -> Result<Vec<Id<Game>>> {
        use crate::schema::game::dsl::*;

        let recs = game
            .filter(simulation_id.eq(sim_id.value()))
            .filter(round.eq(rnd as i64))
            .select(id)
            .load(&mut self.connection)?
            .into_iter()
            .map(|elem: Uuid| elem.into())
            .collect();

        Ok(recs)
    }

    fn games_id_by_team_id(
        &mut self,
        team_id: Id<Team>,
        sim_id: Id<Simulation>,
        cnt: u8,
    ) -> Result<Vec<(Id<Game>, bool)>> {
        use crate::schema::game::dsl::*;

        let mut games = game
            .filter(simulation_id.eq(sim_id.value()))
            .filter(home_team_id.eq(team_id.value()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut self.connection)?
            .into_iter()
            .map(|elem: Uuid| (elem.into(), true))
            .collect::<Vec<(Id<Game>, bool)>>();
        let mut guest_games = game
            .filter(simulation_id.eq(sim_id.value()))
            .filter(guest_team_id.eq(team_id.value()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut self.connection)?
            .into_iter()
            .map(|elem: Uuid| (elem.into(), false))
            .collect();

        games.append(&mut guest_games);

        Ok(games)
    }

    fn h2hs_id_by_team_id(
        &mut self,
        ht_id: Id<Team>,
        gt_id: Id<Team>,
        sim_id: Id<Simulation>,
        cnt: u8,
    ) -> Result<Vec<(Id<Game>, bool)>> {
        use crate::schema::game::dsl::*;

        let mut games = game
            .filter(simulation_id.eq(sim_id.value()))
            .filter(home_team_id.eq(ht_id.value()))
            .filter(guest_team_id.eq(gt_id.value()))
            .select(id)
            .order(round.desc())
            .limit(cnt as i64)
            .load(&mut self.connection)?
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
            .load(&mut self.connection)?
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

use anyhow::Result;
use diesel::prelude::*;
use std::cmp::Ordering;

use crate::DBPool;
use crate::models::GameStatPostrgres;
use application::repository::IGameStatRepo;
use domain::{
    entity::{Game, GameStat},
    value_object::{Id, Winner},
};

impl From<GameStat> for GameStatPostrgres {
    fn from(g: GameStat) -> Self {
        Self {
            id: g.id().value(),
            game_id: g.game_id().value(),
            home_team_total: g.home_team_total() as i16,
            guest_team_total: g.guest_team_total() as i16,
        }
    }
}

impl From<GameStatPostrgres> for GameStat {
    fn from(g: GameStatPostrgres) -> Self {
        Self::new(
            g.id.into(),
            g.game_id.into(),
            g.home_team_total as u8,
            g.guest_team_total as u8,
        )
    }
}

pub struct GameStatRepo {
    pool: DBPool,
}

impl GameStatRepo {
    pub fn new(pool: DBPool) -> Self {
        Self { pool }
    }
}

impl IGameStatRepo for GameStatRepo {
    fn add(&self, game_stat: GameStat) -> Result<()> {
        use crate::schema::gamestat;

        let mut connection = self.pool.get()?;
        let game_stat = GameStatPostrgres::from(game_stat);
        diesel::insert_into(gamestat::table)
            .values(&game_stat)
            .execute(&mut connection)?;

        Ok(())
    }

    fn game_stat_by_game_id(&self, g_id: Id<Game>) -> Result<GameStat> {
        use crate::schema::gamestat::dsl::*;

        let mut connection = self.pool.get()?;
        let rec = gamestat
            .filter(game_id.eq(g_id.value()))
            .select(GameStatPostrgres::as_select())
            .first(&mut connection)?;

        Ok(rec.into())
    }

    fn goals_by_game_id(&self, game_id: Id<Game>, is_home: bool) -> Option<u8> {
        self.score_by_game_id(game_id, is_home).map(|score| score.0)
    }

    fn next_id(&self) -> Id<GameStat> {
        Id::new()
    }

    fn score_by_game_id(&self, g_id: Id<Game>, is_home: bool) -> Option<(u8, u8)> {
        use crate::schema::gamestat::dsl::*;

        let mut connection = self.pool.get().unwrap();
        let rec = gamestat
            .filter(game_id.eq(g_id.value()))
            .select(GameStatPostrgres::as_select())
            .first(&mut connection)
            .ok();
        if let Some(rec) = rec {
            let score = if is_home {
                (rec.home_team_total as u8, rec.guest_team_total as u8)
            } else {
                (rec.guest_team_total as u8, rec.home_team_total as u8)
            };
            Some(score)
        } else {
            None
        }
    }

    fn winner_by_game_id(&self, game_id: Id<Game>, is_home: bool) -> Option<Winner> {
        if let Some((home_team_total, guest_team_total)) = self.score_by_game_id(game_id, is_home) {
            Some(match home_team_total.cmp(&guest_team_total) {
                Ordering::Greater => Winner::W1,
                Ordering::Equal => Winner::X,
                Ordering::Less => Winner::W2,
            })
        } else {
            None
        }
    }
}

use anyhow::Result;
use diesel::prelude::*;
use std::cmp::Ordering;
use uuid::Uuid;

use crate::DBPool;
use crate::models::GameStatSqlite;
use application::repository::IGameStatRepo;
use domain::{
    entity::{Game, GameStat},
    value_object::{Id, Winner},
};

impl From<GameStat> for GameStatSqlite {
    fn from(g: GameStat) -> Self {
        Self {
            id: g.id().value().to_string(),
            game_id: g.game_id().value().to_string(),
            home_team_total: g.home_team_total() as i16,
            guest_team_total: g.guest_team_total() as i16,
        }
    }
}

impl From<GameStatSqlite> for GameStat {
    fn from(g: GameStatSqlite) -> Self {
        Self::new(
            Uuid::parse_str(&g.id).unwrap().into(),
            Uuid::parse_str(&g.game_id).unwrap().into(),
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
        use crate::schema::GameStat;

        let mut connection = self.pool.get()?;
        let game_stat = GameStatSqlite::from(game_stat);
        diesel::insert_into(GameStat::table)
            .values(&game_stat)
            .execute(&mut connection)?;

        Ok(())
    }

    fn game_stat_by_game_id(&self, g_id: Id<Game>) -> Result<GameStat> {
        use crate::schema::GameStat::dsl::*;

        let mut connection = self.pool.get()?;
        let rec = GameStat
            .filter(game_id.eq(g_id.value().to_string()))
            .select(GameStatSqlite::as_select())
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
        use crate::schema::GameStat::dsl::*;

        let mut connection = self.pool.get().unwrap();
        let rec = GameStat
            .filter(game_id.eq(g_id.value().to_string()))
            .select(GameStatSqlite::as_select())
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

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use crate::init_pool;
    use crate::repository::common::{GameFactory, SimulationBuilder, run_migrations};
    use crate::repository::{GameRepo, GameStatRepo, SimulationRepo};
    use application::repository::{IGameRepo, IGameStatRepo, ISimulationRepo};
    use domain::{
        entity::{GameStat, Simulation},
        value_object::{Amount, MIN_BALANCE_AMOUNT},
    };

    #[test]
    fn insert_game_stat() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let game_stat_repo = GameStatRepo::new(pool.clone());
        let sim_repo = SimulationRepo::new(pool.clone());
        let sim_id = sim_repo.next_id();
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 3));
        let balance = Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap();
        let simulation = Simulation::new(sim_id, ip, balance, None);
        sim_repo.add(simulation).unwrap();
        let game_repo = GameRepo::new(pool.clone());
        let game = GameFactory::create_spa_zen_game(simulation.id());
        game_repo.add(game).unwrap();

        let game_stat_id = game_stat_repo.next_id();
        let game_stat = GameStat::new(game_stat_id, game.id(), 2, 0);

        let res = game_stat_repo.add(game_stat);

        assert!(res.is_ok());
    }

    #[test]
    fn score_by_game_id() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let game_stat_repo = GameStatRepo::new(pool.clone());
        let sim_repo = SimulationRepo::new(pool.clone());
        let simulation = SimulationBuilder::new().build();
        sim_repo.add(simulation).unwrap();
        let game_repo = GameRepo::new(pool.clone());
        let game = GameFactory::create_spa_zen_game(simulation.id());
        game_repo.add(game).unwrap();

        let game_stat_id = game_stat_repo.next_id();
        let game_stat = GameStat::new(game_stat_id, game.id(), 2, 0);
        game_stat_repo.add(game_stat).unwrap();

        let score_home = game_stat_repo.score_by_game_id(game.id(), true).unwrap();
        let score_guest = game_stat_repo.score_by_game_id(game.id(), false).unwrap();

        assert_eq!(score_home, (2, 0));
        assert_eq!(score_guest, (0, 2));
    }

    #[test]
    fn goals_by_game_id() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let game_stat_repo = GameStatRepo::new(pool.clone());
        let sim_repo = SimulationRepo::new(pool.clone());
        let simulation = SimulationBuilder::new().build();
        sim_repo.add(simulation).unwrap();
        let game_repo = GameRepo::new(pool.clone());
        let game = GameFactory::create_spa_zen_game(simulation.id());
        game_repo.add(game).unwrap();

        let game_stat_id = game_stat_repo.next_id();
        let game_stat = GameStat::new(game_stat_id, game.id(), 2, 0);
        game_stat_repo.add(game_stat).unwrap();

        let goals_home = game_stat_repo.goals_by_game_id(game.id(), true).unwrap();
        let goals_guest = game_stat_repo.goals_by_game_id(game.id(), false).unwrap();

        assert_eq!(goals_home, 2);
        assert_eq!(goals_guest, 0);
    }
}

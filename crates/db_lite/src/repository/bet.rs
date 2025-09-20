use anyhow::Result;
use diesel::{dsl::min, prelude::*};
use rmp_serde;
use uuid::Uuid;

use crate::DBPool;
use crate::models::BetSqlite;
use application::repository::IBetRepo;
use domain::{
    entity::Bet,
    value_object::{Amount, Coefficient, Id, MIN_BET_AMOUNT},
};

impl From<Bet> for BetSqlite {
    fn from(b: Bet) -> Self {
        Self {
            id: b.id().value().to_string(),
            simulation_id: b.simulation_id().value().to_string(),
            amount: b.amount().clear_value(),
            coefficient: b.coefficient().clear_value(),
            game_id: b.game_id().value().to_string(),
            event: rmp_serde::to_vec(&b.event()).unwrap(),
            is_won: b.is_won(),
        }
    }
}

impl From<BetSqlite> for Bet {
    fn from(b: BetSqlite) -> Self {
        Self::new(
            Uuid::parse_str(&b.id).unwrap().into(),
            Uuid::parse_str(&b.simulation_id).unwrap().into(),
            Amount::new(b.amount, Some(MIN_BET_AMOUNT)).unwrap(),
            b.coefficient.try_into().unwrap(),
            Uuid::parse_str(&b.game_id).unwrap().into(),
            rmp_serde::from_slice(&b.event).unwrap(),
            b.is_won,
        )
    }
}

pub struct BetRepo {
    pool: DBPool,
}

impl BetRepo {
    pub fn new(pool: DBPool) -> Self {
        Self { pool }
    }
}

impl IBetRepo for BetRepo {
    fn add(&self, bet: Bet) -> Result<()> {
        use crate::schema::Bet;

        let mut connection = self.pool.get()?;
        let bet = BetSqlite::from(bet);
        diesel::insert_into(Bet::table)
            .values(&bet)
            .execute(&mut connection)?;

        Ok(())
    }

    fn min_coefficient_lose(&self) -> Option<Coefficient> {
        use crate::schema::Bet::dsl::*;

        let mut connection = self.pool.get().unwrap();
        let value = Bet
            .filter(is_won.eq(Some(false)))
            .select(min(coefficient))
            .first::<Option<i32>>(&mut connection)
            .ok()
            .flatten();

        match value {
            Some(c) => Coefficient::try_from(c).ok(),
            None => None,
        }
    }

    fn next_id(&self) -> Id<Bet> {
        Id::new()
    }

    fn not_calculated_bets(&self) -> Vec<Bet> {
        use crate::schema::Bet::dsl::*;

        let mut connection = self.pool.get().unwrap();
        Bet.filter(is_won.is_null())
            .select(BetSqlite::as_select())
            .load(&mut connection)
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|b: BetSqlite| b.into())
            .collect()
    }

    fn update_status(&self, bet: Bet) -> Result<()> {
        use crate::schema::Bet::{
            self,
            dsl::{id, is_won},
        };

        let mut connection = self.pool.get()?;
        let bet = BetSqlite::from(bet);
        diesel::update(Bet::table)
            .filter(id.eq(&bet.id))
            .set(is_won.eq(bet.is_won))
            .execute(&mut connection)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use crate::init_pool;
    use crate::repository::common::{GameFactory, SimulationBuilder, run_migrations};
    use crate::repository::{BetRepo, GameRepo, SimulationRepo};
    use application::repository::{IBetRepo, IGameRepo, ISimulationRepo};
    use domain::{
        entity::Bet,
        value_object::{Amount, Event, MIN_BALANCE_AMOUNT, MIN_BET_AMOUNT, Winner},
    };

    #[test]
    fn insert_bet() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let bet_repo = BetRepo::new(pool.clone());
        let bet_id = bet_repo.next_id();
        let sim_repo = SimulationRepo::new(pool.clone());
        let simulation = SimulationBuilder::new().round(5).build();
        sim_repo.add(simulation).unwrap();
        let amount = Amount::new(3000, Some(MIN_BET_AMOUNT)).unwrap();
        let coefficient = (2.40).try_into().unwrap();
        let game_repo = GameRepo::new(pool.clone());
        let game = GameFactory::create_spa_zen_game(simulation.id());
        game_repo.add(game).unwrap();
        let event = Event::WDL(Winner::W1);
        let is_won = None;
        let bet = Bet::new(
            bet_id,
            simulation.id(),
            amount,
            coefficient,
            game.id(),
            event,
            is_won,
        );

        let res = bet_repo.add(bet);

        assert!(res.is_ok());
    }

    #[test]
    fn min_coefficient_lose() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let bet_repo = BetRepo::new(pool.clone());
        let sim_repo = SimulationRepo::new(pool.clone());
        let simulation = SimulationBuilder::new()
            .ip(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)))
            .balance(Amount::new(0, Some(MIN_BALANCE_AMOUNT)).unwrap())
            .build();
        sim_repo.add(simulation).unwrap();
        let game_repo = GameRepo::new(pool.clone());
        let game = GameFactory::create_spa_zen_game(simulation.id());
        game_repo.add(game).unwrap();

        let bet_id = bet_repo.next_id();
        let amount = Amount::new(3000, Some(MIN_BET_AMOUNT)).unwrap();
        let coefficient = (2.40).try_into().unwrap();
        let event = Event::WDL(Winner::W1);
        let is_won = Some(false);
        let bet = Bet::new(
            bet_id,
            simulation.id(),
            amount,
            coefficient,
            game.id(),
            event,
            is_won,
        );
        bet_repo.add(bet).unwrap();

        let bet_id = bet_repo.next_id();
        let amount = Amount::new(2000, Some(MIN_BET_AMOUNT)).unwrap();
        let coefficient = (2.30).try_into().unwrap();
        let event = Event::WDL(Winner::W1);
        let is_won = Some(false);
        let bet = Bet::new(
            bet_id,
            simulation.id(),
            amount,
            coefficient,
            game.id(),
            event,
            is_won,
        );
        bet_repo.add(bet).unwrap();

        let bet_id = bet_repo.next_id();
        let amount = Amount::new(2000, Some(MIN_BET_AMOUNT)).unwrap();
        let coefficient = (2.50).try_into().unwrap();
        let event = Event::WDL(Winner::W1);
        let is_won = Some(false);
        let bet = Bet::new(
            bet_id,
            simulation.id(),
            amount,
            coefficient,
            game.id(),
            event,
            is_won,
        );
        bet_repo.add(bet).unwrap();

        let res = bet_repo.min_coefficient_lose();

        assert_eq!(res, Some((2.30).try_into().unwrap()));
    }

    #[test]
    fn not_calculated_bets() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let bet_repo = BetRepo::new(pool.clone());
        let sim_repo = SimulationRepo::new(pool.clone());
        let simulation = SimulationBuilder::new()
            .ip(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 3)))
            .build();
        sim_repo.add(simulation).unwrap();
        let game_repo = GameRepo::new(pool.clone());
        let game = GameFactory::create_spa_zen_game(simulation.id());
        game_repo.add(game).unwrap();

        let bet_id = bet_repo.next_id();
        let amount = Amount::new(3000, Some(MIN_BET_AMOUNT)).unwrap();
        let coefficient = (2.40).try_into().unwrap();
        let event = Event::WDL(Winner::W1);
        let is_won = None;
        let bet1 = Bet::new(
            bet_id,
            simulation.id(),
            amount,
            coefficient,
            game.id(),
            event,
            is_won,
        );
        bet_repo.add(bet1).unwrap();

        let bet_id = bet_repo.next_id();
        let amount = Amount::new(2000, Some(MIN_BET_AMOUNT)).unwrap();
        let coefficient = (2.30).try_into().unwrap();
        let game = GameFactory::create_din_lok_game(simulation.id());
        let event = Event::WDL(Winner::W1);
        let is_won = Some(false);
        let bet2 = Bet::new(
            bet_id,
            simulation.id(),
            amount,
            coefficient,
            game.id(),
            event,
            is_won,
        );
        bet_repo.add(bet2).unwrap();

        let bet_id = bet_repo.next_id();
        let amount = Amount::new(2000, Some(MIN_BET_AMOUNT)).unwrap();
        let coefficient = (2.50).try_into().unwrap();
        let game = GameFactory::create_kra_ros_game(simulation.id());
        let event = Event::WDL(Winner::W1);
        let is_won = None;
        let bet3 = Bet::new(
            bet_id,
            simulation.id(),
            amount,
            coefficient,
            game.id(),
            event,
            is_won,
        );
        bet_repo.add(bet3).unwrap();

        let res = bet_repo.not_calculated_bets();

        assert_eq!(res.len(), 2);
    }
}

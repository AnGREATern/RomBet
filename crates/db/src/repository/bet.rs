use anyhow::Result;
use diesel::{dsl::min, prelude::*};
use rmp_serde;

use crate::DBPool;
use crate::models::BetPostgres;
use application::repository::IBetRepo;
use domain::{
    entity::Bet,
    value_object::{Amount, Coefficient, Id, MIN_BET_AMOUNT},
};

impl From<Bet> for BetPostgres {
    fn from(b: Bet) -> Self {
        Self {
            id: b.id().value(),
            simulation_id: b.simulation_id().value(),
            amount: b.amount().clear_value(),
            coefficient: b.coefficient().clear_value(),
            game_id: b.game_id().value(),
            event: rmp_serde::to_vec(&b.event()).unwrap(),
            is_won: b.is_won(),
        }
    }
}

impl From<BetPostgres> for Bet {
    fn from(b: BetPostgres) -> Self {
        Self::new(
            b.id.into(),
            b.simulation_id.into(),
            Amount::new(b.amount, Some(MIN_BET_AMOUNT)).unwrap(),
            b.coefficient.try_into().unwrap(),
            b.game_id.into(),
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
        use crate::schema::bet;

        let mut connection = self.pool.get()?;
        let bet = BetPostgres::from(bet);
        diesel::insert_into(bet::table)
            .values(&bet)
            .execute(&mut connection)?;

        Ok(())
    }

    fn min_coefficient_lose(&self) -> Option<Coefficient> {
        use crate::schema::bet::dsl::*;

        let mut connection = self.pool.get().unwrap();
        let value = bet
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
        use crate::schema::bet::dsl::*;

        let mut connection = self.pool.get().unwrap();
        bet.filter(is_won.is_null())
            .select(BetPostgres::as_select())
            .load(&mut connection)
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|b: BetPostgres| b.into())
            .collect()
    }

    fn update_status(&self, bet: Bet) -> Result<()> {
        use crate::schema::bet::{
            self,
            dsl::{id, is_won},
        };

        let mut connection = self.pool.get()?;
        let bet = BetPostgres::from(bet);
        diesel::update(bet::table)
            .filter(id.eq(&bet.id))
            .set(is_won.eq(bet.is_won))
            .execute(&mut connection)?;

        Ok(())
    }
}

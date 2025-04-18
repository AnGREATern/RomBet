use crate::{establish_connection, models::BetPostrgres};
use application::repository::IBetRepo;
use domain::{
    entity::Bet,
    value_object::{Coefficient, Id},
};

use anyhow::Result;
use diesel::{dsl::min, prelude::*};
use rmp_serde;

impl From<Bet> for BetPostrgres {
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

impl From<BetPostrgres> for Bet {
    fn from(b: BetPostrgres) -> Self {
        Self::new(
            b.id.into(),
            b.simulation_id.into(),
            b.amount.try_into().unwrap(),
            b.coefficient.try_into().unwrap(),
            b.game_id.into(),
            rmp_serde::from_slice(&b.event).unwrap(),
            b.is_won,
        )
    }
}

pub struct BetRepo {
    connection: PgConnection,
}

impl IBetRepo for BetRepo {
    fn new() -> Self {
        let connection = establish_connection();
        Self { connection }
    }

    fn add(&mut self, bet: Bet) -> Result<()> {
        use crate::schema::bet;

        let bet = BetPostrgres::from(bet);
        diesel::insert_into(bet::table)
            .values(&bet)
            .execute(&mut self.connection)?;

        Ok(())
    }

    fn min_coefficient_lose(&mut self) -> Option<Coefficient> {
        use crate::schema::bet::dsl::*;

        let value = bet
            .filter(is_won.eq(Some(false)))
            .select(min(coefficient))
            .first::<Option<i32>>(&mut self.connection)
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

    fn not_calculated_bets(&mut self) -> Vec<Bet> {
        use crate::schema::bet::dsl::*;

        bet.filter(is_won.is_null())
            .select(BetPostrgres::as_select())
            .load(&mut self.connection)
            .ok()
            .unwrap_or_default()
            .into_iter()
            .map(|b: BetPostrgres| b.into())
            .collect()
    }

    fn update_status(&mut self, bet: Bet) -> Result<()> {
        use crate::schema::bet::{
            self,
            dsl::{id, is_won},
        };

        let bet = BetPostrgres::from(bet);
        diesel::update(bet::table)
            .filter(id.eq(&bet.id))
            .set(is_won.eq(bet.is_won))
            .execute(&mut self.connection)?;

        Ok(())
    }
}

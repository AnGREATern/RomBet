use anyhow::Result;

use domain::{
    entity::Bet,
    value_object::{Coefficient, Id},
};

pub trait IBetRepo {
    fn new() -> Self;

    fn add(&mut self, bet: Bet) -> Result<()>;

    fn update_status(&mut self, bet: Bet) -> Result<()>;

    fn min_coefficient_lose(&mut self) -> Option<Coefficient>;

    fn not_calculated_bets(&mut self) -> Vec<Bet>;

    fn next_id(&self) -> Id<Bet>;
}

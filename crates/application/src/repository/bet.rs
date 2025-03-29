use anyhow::Result;

use domain::{
    entity::Bet,
    value_object::{Coefficient, Id},
};

pub trait IBetRepo {
    fn new() -> Self;

    fn add(&self, bet: Bet) -> Result<()>;

    fn update(&self, bet: Bet) -> Result<()>;

    fn min_coefficient_lose(&self) -> Option<Coefficient>;

    fn not_calculated_bets(&self) -> Vec<Bet>;

    fn next_id(&self) -> Id<Bet>;
}

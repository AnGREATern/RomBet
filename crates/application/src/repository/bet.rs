use anyhow::Result;

use domain::{
    entity::Bet,
    value_object::{Amount, Coefficient, Id},
};

pub trait IBetRepo {
    fn new() -> Self;

    fn add(&self, bet: Bet) -> Result<()>;

    fn update(&self, bet: Bet) -> Result<()>;

    fn min_coefficient_lose(&self) -> Option<Coefficient>;

    fn not_calculated_bets(&self) -> Vec<Bet>;

    fn bet_by_id(&self, bet_id: Id<Bet>) -> Result<Bet>;

    fn stat_by_round(&self, round: u32) -> Option<Amount>;

    fn stat_by_bet_cnt(&self, cnt: u32) -> Option<Amount>;

    fn next_id(&self) -> Id<Bet>;

    fn reset(&self);
}

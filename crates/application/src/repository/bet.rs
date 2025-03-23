use anyhow::Result;

use domain::{
    entity::Bet,
    value_object::{Id, Amount},
};

pub trait IBetRepo {
    fn new() -> Self;

    fn add(&self, bet: Bet) -> Result<Id<Bet>>;

    fn bet_by_id(&self, bet_id: Id<Bet>) -> Result<Bet>;

    fn stat_by_round(&self, round: u32) -> Option<Amount>;

    fn stat_by_bet_cnt(&self, cnt: u32) -> Option<Amount>;

    fn next_id(&self) -> Id<Bet>;

    fn reset(&self);
}
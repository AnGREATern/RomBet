use anyhow::Result;
use domain::{entity::Bet, value_object::{Id, Amount}};
use crate::repository::{IGameRepo, IGameStatRepo, IBetRepo};

pub trait IBetService<T: IBetRepo> {
    fn new(repo: T) -> Self;

    fn make_bet(&self) -> Result<()>;

    fn is_success(&self, bet_id: Id<Bet>) -> Result<bool>;

    fn profit(&self, bet_id: Id<Bet>) -> Result<Amount>;

    fn reset(&mut self);
}

pub struct BetService<T: IBetRepo> {
    bet_repo: T,
}

impl<T: IBetRepo> IBetService<T> for BetService<T> {
    fn new(bet_repo: T) -> Self {
        Self { bet_repo }
    }

    fn make_bet(&self) -> Result<()> {
        todo!()
    }

    fn is_success(&self, bet_id: Id<Bet>) -> Result<bool> {
        todo!()
    }

    fn profit(&self, bet_id: Id<Bet>) -> Result<Amount> {
        todo!()
    }

    fn reset(&mut self) {
        self.bet_repo.reset();
    }
}
use std::cmp::Ordering;
use anyhow::Result;

use domain::{entity::{Bet, Team}, value_object::{Amount, Id}};
use crate::{repository::{IBetRepo, IGameRepo, IGameStatRepo}, usecase::bet::{IBetService, MakeBet, MakeReport}};

pub struct BetService<T: IBetRepo> {
    bet_repo: T,
}

impl<T: IBetRepo> MakeBet for BetService<T> {
    fn make_bet(&self) -> Result<()> {
        todo!()
    }
}

impl<T: IBetRepo> MakeReport for BetService<T> {
    fn make_report(&self) -> Result<()> {
        todo!()
    }
}

impl<T: IBetRepo> IBetService for BetService<T> { }

impl<T: IBetRepo> BetService<T> {
    fn new(bet_repo: T) -> Self {
        Self { bet_repo }
    }

    fn past_total_cnt_by_team_id(&self, team_id: Id<Team>, total: u8, ordering: Ordering) -> u8 {
        todo!()
    }
}

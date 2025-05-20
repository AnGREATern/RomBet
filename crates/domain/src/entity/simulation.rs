use std::net::IpAddr;
use anyhow::Result;

use crate::value_object::{Amount, Id, MIN_BALANCE_AMOUNT};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Simulation {
    id: Id<Simulation>,
    ip: IpAddr,
    round: u32,
    balance: Amount,
}

impl Simulation {
    pub fn new(id: Id<Simulation>, ip: IpAddr, balance: Amount) -> Self {
        let round = 0;
        Self {
            id,
            ip,
            round,
            balance,
        }
    }

    pub fn id(&self) -> Id<Self> {
        self.id
    }

    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    pub fn round(&self) -> u32 {
        self.round
    }

    pub fn balance(&self) -> Amount {
        self.balance
    }

    pub fn process_bet(&mut self, bet_res: Amount) -> Result<Amount> {
        self.balance = Amount::new(self.balance.clear_value() + bet_res.clear_value(), Some(MIN_BALANCE_AMOUNT))?;

        Ok(self.balance)
    }

    pub fn make_bet(&mut self, bet_amount: Amount) -> Result<Amount> {
        self.balance = Amount::new(self.balance.clear_value() - bet_amount.clear_value(), Some(MIN_BALANCE_AMOUNT))?;

        Ok(self.balance)
    }

    pub fn increment_round(&mut self) {
        self.round += 1;
    }
}

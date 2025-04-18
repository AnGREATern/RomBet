use std::net::IpAddr;

use crate::value_object::{Amount, Id};

pub struct Simulation {
    id: Id<Simulation>,
    ip: IpAddr,
    round: u32,
    balance: Amount,
}

impl Simulation {
    pub fn new(id: Id<Simulation>, ip: IpAddr, balance: Amount) -> Self {
        let round = 1;
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

    pub fn increment_round(&mut self) {
        self.round += 1;
    }
}

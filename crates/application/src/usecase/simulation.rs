use std::net::IpAddr;

use anyhow::Result;
use domain::{entity::Simulation, value_object::Id};

pub trait CreateRound {
    fn create_round(&mut self, simulation: &mut Simulation) -> Result<()>;
}

pub trait Start {
    fn start(&self, ip: IpAddr) -> Id<Simulation>;

    fn restart(&self, simulation: Simulation) -> Id<Simulation>;
}

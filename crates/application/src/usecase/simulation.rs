use std::net::IpAddr;

use anyhow::Result;
use domain::{entity::Simulation, value_object::Id};

pub trait CreateRound {
    fn create_round(&mut self, simulation: &mut Simulation) -> Result<()>;
}

pub trait Start {
    fn start(&mut self, ip: IpAddr) -> Result<Id<Simulation>>;

    fn restart(&mut self, simulation: Simulation) -> Result<Id<Simulation>>;
}

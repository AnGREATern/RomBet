use std::net::IpAddr;
use anyhow::Result;

use domain::{entity::Simulation, value_object::Id};
use crate::service::DisplayedGame;

pub trait CreateRound {
    fn create_round(&mut self, simulation: &mut Simulation) -> Result<Vec<DisplayedGame>>;
}

pub trait Start {
    fn start(&mut self, ip: IpAddr) -> Result<Simulation>;

    fn restart(&mut self, simulation_id: Id<Simulation>) -> Result<Simulation>;
}

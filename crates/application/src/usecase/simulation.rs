use anyhow::Result;
use std::net::IpAddr;

use crate::service::DisplayedGame;
use domain::{entity::Simulation, value_object::Id};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait CreateRound {
    fn create_round(&self, simulation: &mut Simulation) -> Result<Vec<DisplayedGame>>;
}

#[cfg_attr(test, automock)]
pub trait Start {
    fn start(&self, ip: IpAddr) -> Result<Simulation>;

    fn restart(&self, simulation_id: Id<Simulation>) -> Result<Simulation>;
}

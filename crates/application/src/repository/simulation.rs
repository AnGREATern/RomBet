use anyhow::Result;
use std::net::IpAddr;

use domain::{entity::Simulation, value_object::Id};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait ISimulationRepo {
    fn add(&self, simulation: Simulation) -> Result<()>;

    fn simulation_by_ip(&self, ip: IpAddr) -> Option<Simulation>;

    fn simulation_by_id(&self, id: Id<Simulation>) -> Result<Simulation>;

    fn remove_by_id(&self, simulation_id: Id<Simulation>);

    fn update_by_id(&self, simulation: Simulation) -> Result<()>;

    fn next_id(&self) -> Id<Simulation>;
}

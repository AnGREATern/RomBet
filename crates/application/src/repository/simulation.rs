use std::net::IpAddr;
use anyhow::Result;

use domain::{entity::Simulation, value_object::Id};

pub trait ISimulationRepo {
    fn new() -> Self;

    fn add(&mut self, simulation: Simulation) -> Result<()>;

    fn simulation_by_ip(&mut self, ip: IpAddr) -> Option<Id<Simulation>>;

    fn remove_by_id(&mut self, simulation_id: Id<Simulation>);

    fn next_id(&self) -> Id<Simulation>;
}

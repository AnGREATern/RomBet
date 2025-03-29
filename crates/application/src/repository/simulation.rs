use std::net::IpAddr;

use domain::{entity::Simulation, value_object::Id};

pub trait ISimulationRepo {
    fn new() -> Self;

    fn add(&self, simulation: Simulation);

    fn simulation_by_ip(&self, ip: IpAddr) -> Option<Id<Simulation>>;

    fn remove_by_id(&self, simulation_id: Id<Simulation>);

    fn next_id(&self) -> Id<Simulation>;
}

use anyhow::Result;

use domain::{entity::Simulation, value_object::Id};

pub trait ISimulationRepo {
    fn new() -> Self;

    fn session_by_id(&self, session_id: Id<Simulation>) -> Result<Simulation>;

    fn remove_by_id(&self, simulation_id: Id<Simulation>);

    fn next_id(&self) -> Id<Simulation>;
}

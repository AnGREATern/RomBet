use anyhow::Result;
use domain::entity::Simulation;

pub trait CreateRound {
    fn create_round(&mut self, simulation: &mut Simulation) -> Result<()>;
}

pub trait Restart {
    fn restart(&self, simulation: Simulation) -> Simulation;
}

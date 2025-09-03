use anyhow::Result;
use diesel::prelude::*;
use std::net::IpAddr;

use crate::DBPool;
use crate::models::SimulationPostrgres;
use application::repository::ISimulationRepo;
use domain::{
    entity::Simulation,
    value_object::{Amount, Id, MIN_BALANCE_AMOUNT},
};

impl From<Simulation> for SimulationPostrgres {
    fn from(s: Simulation) -> Self {
        Self {
            id: s.id().value(),
            ip: s.ip().to_string(),
            round: s.round() as i64,
            balance: s.balance().clear_value(),
        }
    }
}

impl From<SimulationPostrgres> for Simulation {
    fn from(s: SimulationPostrgres) -> Self {
        Simulation::new(
            s.id.into(),
            s.ip.parse().unwrap(),
            Amount::new(s.balance, Some(MIN_BALANCE_AMOUNT)).unwrap(),
        )
    }
}

pub struct SimulationRepo {
    pool: DBPool,
}

impl SimulationRepo {
    pub fn new(pool: DBPool) -> Self {
        Self { pool }
    }
}

impl ISimulationRepo for SimulationRepo {
    fn add(&self, simulation: Simulation) -> Result<()> {
        use crate::schema::simulation;

        let mut connection = self.pool.get()?;
        let simulation = SimulationPostrgres::from(simulation);
        diesel::insert_into(simulation::table)
            .values(&simulation)
            .execute(&mut connection)?;

        Ok(())
    }

    fn next_id(&self) -> Id<Simulation> {
        Id::new()
    }

    fn remove_by_id(&self, simulation_id: Id<Simulation>) {
        use crate::schema::simulation::dsl::*;

        let mut connection = self.pool.get().unwrap();
        let _ = diesel::delete(simulation.filter(id.eq(simulation_id.value())))
            .execute(&mut connection);
    }

    fn simulation_by_ip(&self, ip_addr: IpAddr) -> Option<Simulation> {
        use crate::schema::simulation::dsl::*;

        let mut connection = self.pool.get().unwrap();
        let ip_addr = ip_addr.to_string();
        let rec = simulation
            .filter(ip.eq(&ip_addr))
            .select(SimulationPostrgres::as_select())
            .first::<SimulationPostrgres>(&mut connection)
            .ok();

        match rec {
            Some(sim) => Some(sim.into()),
            None => None,
        }
    }

    fn simulation_by_id(&self, sim_id: Id<Simulation>) -> Result<Simulation> {
        use crate::schema::simulation::dsl::*;

        let mut connection = self.pool.get()?;
        let rec = simulation
            .filter(id.eq(&sim_id.value()))
            .select(SimulationPostrgres::as_select())
            .first::<SimulationPostrgres>(&mut connection)?;

        Ok(rec.into())
    }

    fn update_by_id(&self, simulation: Simulation) -> Result<()> {
        use crate::schema::simulation::{
            self,
            dsl::{balance, id, round},
        };

        let mut connection = self.pool.get()?;
        diesel::update(simulation::table)
            .filter(id.eq(&simulation.id().value()))
            .set((
                round.eq(simulation.round() as i64),
                balance.eq(simulation.balance().clear_value()),
            ))
            .execute(&mut connection)?;

        Ok(())
    }
}

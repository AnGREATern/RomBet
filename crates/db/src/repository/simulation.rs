use crate::{establish_connection, models::SimulationPostrgres};
use application::repository::ISimulationRepo;
use domain::{entity::Simulation, value_object::Id};

use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;
use std::net::IpAddr;

pub struct SimulationRepo {
    connection: PgConnection,
}

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
            s.balance.try_into().unwrap(),
        )
    }
}

impl ISimulationRepo for SimulationRepo {
    fn new() -> Self {
        let connection = establish_connection();
        Self { connection }
    }

    fn add(&mut self, simulation: Simulation) -> Result<()> {
        use crate::schema::simulation;

        let simulation = SimulationPostrgres::from(simulation);
        diesel::insert_into(simulation::table)
            .values(&simulation)
            .execute(&mut self.connection)?;

        Ok(())
    }

    fn next_id(&self) -> Id<Simulation> {
        Id::new()
    }

    fn remove_by_id(&mut self, simulation_id: Id<Simulation>) {
        use crate::schema::simulation::dsl::*;

        let _ = diesel::delete(simulation.filter(id.eq(simulation_id.value())))
            .execute(&mut self.connection);
    }

    fn simulation_by_ip(&mut self, ip_addr: IpAddr) -> Option<Id<Simulation>> {
        use crate::schema::simulation::dsl::*;

        let ip_addr = ip_addr.to_string();
        let rec = simulation
            .filter(ip.eq(&ip_addr))
            .select(id)
            .first::<Uuid>(&mut self.connection)
            .ok();

        match rec {
            Some(s_id) => Some(s_id.into()),
            None => None,
        }
    }
}

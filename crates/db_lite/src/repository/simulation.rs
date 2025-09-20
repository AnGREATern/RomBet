use anyhow::Result;
use diesel::prelude::*;
use std::net::IpAddr;
use uuid::Uuid;

use crate::DBPool;
use crate::models::SimulationSqlite;
use application::repository::ISimulationRepo;
use domain::{
    entity::Simulation,
    value_object::{Amount, Id, MIN_BALANCE_AMOUNT},
};

impl From<Simulation> for SimulationSqlite {
    fn from(s: Simulation) -> Self {
        Self {
            id: s.id().value().to_string(),
            ip: s.ip().to_string(),
            round: s.round() as i64,
            balance: s.balance().clear_value(),
        }
    }
}

impl From<SimulationSqlite> for Simulation {
    fn from(s: SimulationSqlite) -> Self {
        Simulation::new(
            Uuid::parse_str(&s.id).unwrap().into(),
            s.ip.parse().unwrap(),
            Amount::new(s.balance, Some(MIN_BALANCE_AMOUNT)).unwrap(),
            Some(s.round as u32),
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
        use crate::schema::Simulation;

        let mut connection = self.pool.get()?;
        let simulation = SimulationSqlite::from(simulation);
        diesel::insert_into(Simulation::table)
            .values(&simulation)
            .execute(&mut connection)?;

        Ok(())
    }

    fn next_id(&self) -> Id<Simulation> {
        Id::new()
    }

    fn remove_by_id(&self, simulation_id: Id<Simulation>) {
        use crate::schema::Simulation::dsl::*;

        let mut connection = self.pool.get().unwrap();
        let _ = diesel::delete(Simulation.filter(id.eq(simulation_id.value().to_string())))
            .execute(&mut connection);
    }

    fn simulation_by_ip(&self, ip_addr: IpAddr) -> Option<Simulation> {
        use crate::schema::Simulation::dsl::*;

        let mut connection = self.pool.get().unwrap();
        let ip_addr = ip_addr.to_string();
        let rec = Simulation
            .filter(ip.eq(&ip_addr))
            .select(SimulationSqlite::as_select())
            .first::<SimulationSqlite>(&mut connection)
            .ok();

        rec.map(|sim| sim.into())
    }

    fn simulation_by_id(&self, sim_id: Id<Simulation>) -> Result<Simulation> {
        use crate::schema::Simulation::dsl::*;

        let mut connection = self.pool.get()?;
        let rec = Simulation
            .filter(id.eq(&sim_id.value().to_string()))
            .select(SimulationSqlite::as_select())
            .first::<SimulationSqlite>(&mut connection)?;

        Ok(rec.into())
    }

    fn update_by_id(&self, simulation: Simulation) -> Result<()> {
        use crate::schema::Simulation::{
            self,
            dsl::{balance, id, round},
        };

        let mut connection = self.pool.get()?;
        diesel::update(Simulation::table)
            .filter(id.eq(&simulation.id().value().to_string()))
            .set((
                round.eq(simulation.round() as i64),
                balance.eq(simulation.balance().clear_value()),
            ))
            .execute(&mut connection)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use crate::repository::SimulationRepo;
    use crate::repository::common::run_migrations;
    use crate::{init_pool, repository::common::SimulationBuilder};
    use application::repository::ISimulationRepo;

    #[test]
    fn add_get_remove() {
        let pool = init_pool();
        run_migrations(&mut pool.get().unwrap());

        let repo = SimulationRepo::new(pool);
        let id = repo.next_id();
        let ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let simulation = SimulationBuilder::new().id(id).ip(ip).build();

        repo.add(simulation).unwrap();
        let rec = repo.simulation_by_ip(ip);
        repo.remove_by_id(id);

        assert!(rec.unwrap().id() == id);
    }
}

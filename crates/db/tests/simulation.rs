use std::net::{IpAddr, Ipv4Addr};

use application::repository::ISimulationRepo;
use db::init_pool;
use db::repository::SimulationRepo;
use domain::{
    entity::Simulation,
    value_object::{Amount, MIN_BALANCE_AMOUNT},
};

#[test]
fn add_get_remove() {
    let pool = init_pool();

    let repo = SimulationRepo::new(pool);
    let id = repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let balance = Amount::new(100, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let simulation = Simulation::new(id, ip, balance, None);

    repo.add(simulation).unwrap();
    let rec = repo.simulation_by_ip(ip);
    repo.remove_by_id(id);

    assert!(rec.unwrap().id() == id);
}

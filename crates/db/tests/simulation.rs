use std::net::{IpAddr, Ipv4Addr};

use application::repository::ISimulationRepo;
use db::repository::SimulationRepo;
use domain::{entity::Simulation, value_object::Amount};

#[test]
fn add_get_remove() {
    let mut repo = SimulationRepo::new();
    let id = repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let balance = Amount::try_from(1000.).unwrap();
    let simulation = Simulation::new(id, ip, balance);

    repo.add(simulation).unwrap();
    let rec = repo.simulation_by_ip(ip);
    repo.remove_by_id(id);

    assert!(rec.unwrap() == id);
}
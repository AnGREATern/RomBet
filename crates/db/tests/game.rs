use std::net::{IpAddr, Ipv4Addr};

use application::repository::{IGameRepo, ISimulationRepo, ITeamRepo};
use db::repository::{GameRepo, SimulationRepo, TeamRepo};
use domain::{
    entity::{Game, Simulation},
    value_object::{Amount, MIN_BALANCE_AMOUNT},
};

#[test]
fn game_by_id_found() {
    let mut game_repo = GameRepo::new();
    let mut sim_repo = SimulationRepo::new();
    let sim_id = sim_repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let balance = Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let simulation = Simulation::new(sim_id, ip, balance);
    sim_repo.add(simulation).unwrap();
    let game_id = game_repo.next_id();
    let mut team_repo = TeamRepo::new();
    let team_ids = team_repo.all_teams_id();
    let game = Game::new(game_id, sim_id, team_ids[0], team_ids[1], 1);
    game_repo.add(game).unwrap();
    
    let rec = game_repo.game_by_id(game_id);

    assert!(rec.is_ok());

    sim_repo.remove_by_id(sim_id);
}

#[test]
fn game_by_id_did_not_found() {
    let mut game_repo = GameRepo::new();
    let mut sim_repo = SimulationRepo::new();
    let sim_id = sim_repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let balance = Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let simulation = Simulation::new(sim_id, ip, balance);
    sim_repo.add(simulation).unwrap();
    let game_id = game_repo.next_id();
    let mut team_repo = TeamRepo::new();
    let team_ids = team_repo.all_teams_id();
    let game = Game::new(game_id, sim_id, team_ids[0], team_ids[1], 1);
    game_repo.add(game).unwrap();
    
    let rec = game_repo.game_by_id(sim_id.value().into());

    assert!(rec.is_err());

    sim_repo.remove_by_id(sim_id);
}

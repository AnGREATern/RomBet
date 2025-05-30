use std::net::{IpAddr, Ipv4Addr};

use application::repository::{IGameStatRepo, IGameRepo, ISimulationRepo, ITeamRepo};
use db::repository::{GameStatRepo, GameRepo, SimulationRepo, TeamRepo};
use domain::{entity::{Game, GameStat, Simulation}, value_object::{Amount, MIN_BALANCE_AMOUNT}};

#[test]
fn insert_game_stat() {
    let mut game_stat_repo = GameStatRepo::new();
    let mut sim_repo = SimulationRepo::new();
    let sim_id = sim_repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let balance = Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let simulation = Simulation::new(sim_id, ip, balance);
    sim_repo.add(simulation).unwrap();
    let mut game_repo = GameRepo::new();
    let game_id = game_repo.next_id();
    let mut team_repo = TeamRepo::new();
    let team_ids = team_repo.all_teams_id();
    let game = Game::new(game_id, sim_id, team_ids[0], team_ids[1], 1);
    game_repo.add(game).unwrap();

    let game_stat_id = game_stat_repo.next_id();
    let game_stat = GameStat::new(game_stat_id, game_id, 2, 0);

    let res = game_stat_repo.add(game_stat);

    assert!(res.is_ok());

    sim_repo.remove_by_id(sim_id);
}

#[test]
fn score_by_game_id() {
    let mut game_stat_repo = GameStatRepo::new();
    let mut sim_repo = SimulationRepo::new();
    let sim_id = sim_repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let balance = Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let simulation = Simulation::new(sim_id, ip, balance);
    sim_repo.add(simulation).unwrap();
    let mut game_repo = GameRepo::new();
    let game_id = game_repo.next_id();
    let mut team_repo = TeamRepo::new();
    let team_ids = team_repo.all_teams_id();
    let game = Game::new(game_id, sim_id, team_ids[0], team_ids[1], 1);
    game_repo.add(game).unwrap();

    let game_stat_id = game_stat_repo.next_id();
    let game_stat = GameStat::new(game_stat_id, game_id, 2, 0);
    game_stat_repo.add(game_stat).unwrap();

    let score_home = game_stat_repo.score_by_game_id(game_id, true).unwrap();
    let score_guest = game_stat_repo.score_by_game_id(game_id, false).unwrap();

    assert_eq!(score_home, (2, 0));
    assert_eq!(score_guest, (0, 2));

    sim_repo.remove_by_id(sim_id);
}

#[test]
fn goals_by_game_id() {
    let mut game_stat_repo = GameStatRepo::new();
    let mut sim_repo = SimulationRepo::new();
    let sim_id = sim_repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let balance = Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let simulation = Simulation::new(sim_id, ip, balance);
    sim_repo.add(simulation).unwrap();
    let mut game_repo = GameRepo::new();
    let game_id = game_repo.next_id();
    let mut team_repo = TeamRepo::new();
    let team_ids = team_repo.all_teams_id();
    let game = Game::new(game_id, sim_id, team_ids[0], team_ids[1], 1);
    game_repo.add(game).unwrap();

    let game_stat_id = game_stat_repo.next_id();
    let game_stat = GameStat::new(game_stat_id, game_id, 2, 0);
    game_stat_repo.add(game_stat).unwrap();

    let goals_home = game_stat_repo.goals_by_game_id(game_id, true).unwrap();
    let goals_guest = game_stat_repo.goals_by_game_id(game_id, false).unwrap();

    assert_eq!(goals_home, 2);
    assert_eq!(goals_guest, 0);

    sim_repo.remove_by_id(sim_id);
}
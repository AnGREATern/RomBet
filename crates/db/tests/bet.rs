use std::net::{IpAddr, Ipv4Addr};

use application::repository::{IBetRepo, IGameRepo, ISimulationRepo, ITeamRepo};
use db::init_pool;
use db::repository::{BetRepo, GameRepo, SimulationRepo, TeamRepo};
use domain::{
    entity::{Bet, Game, Simulation},
    value_object::{Amount, Event, MIN_BALANCE_AMOUNT, MIN_BET_AMOUNT, Winner},
};

#[test]
fn insert_bet() {
    let pool = init_pool();

    let mut bet_repo = BetRepo::new(pool.clone());
    let bet_id = bet_repo.next_id();
    let mut sim_repo = SimulationRepo::new(pool.clone());
    let sim_id = sim_repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 100, 0, 1));
    let balance = Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let simulation = Simulation::new(sim_id, ip, balance);
    sim_repo.add(simulation).unwrap();
    let amount = Amount::new(3000, Some(MIN_BET_AMOUNT)).unwrap();
    let coefficient = (2.40).try_into().unwrap();
    let mut game_repo = GameRepo::new(pool.clone());
    let game_id = game_repo.next_id();
    let mut team_repo = TeamRepo::new(pool.clone());
    let team_ids = team_repo.all_teams_id();
    let game = Game::new(game_id, sim_id, team_ids[0], team_ids[1], 1);
    game_repo.add(game).unwrap();
    let event = Event::WDL(Winner::W1);
    let is_won = None;
    let bet = Bet::new(bet_id, sim_id, amount, coefficient, game_id, event, is_won);

    let res = bet_repo.add(bet);

    assert!(res.is_ok());

    sim_repo.remove_by_id(sim_id);
}

#[test]
fn min_coefficient_lose() {
    let pool = init_pool();

    let mut bet_repo = BetRepo::new(pool.clone());
    let mut sim_repo = SimulationRepo::new(pool.clone());
    let sim_id = sim_repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let balance = Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let simulation = Simulation::new(sim_id, ip, balance);
    sim_repo.add(simulation).unwrap();
    let mut game_repo = GameRepo::new(pool.clone());
    let game_id = game_repo.next_id();
    let mut team_repo = TeamRepo::new(pool);
    let team_ids = team_repo.all_teams_id();
    let game = Game::new(game_id, sim_id, team_ids[0], team_ids[1], 1);
    game_repo.add(game).unwrap();

    let bet_id = bet_repo.next_id();
    let amount = Amount::new(300, Some(MIN_BET_AMOUNT)).unwrap();
    let coefficient = (2.40).try_into().unwrap();
    let event = Event::WDL(Winner::W1);
    let is_won = Some(false);
    let bet = Bet::new(bet_id, sim_id, amount, coefficient, game_id, event, is_won);
    bet_repo.add(bet).unwrap();

    let bet_id = bet_repo.next_id();
    let amount = Amount::new(300, Some(MIN_BET_AMOUNT)).unwrap();
    let coefficient = (2.30).try_into().unwrap();
    let event = Event::WDL(Winner::W1);
    let is_won = Some(false);
    let bet = Bet::new(bet_id, sim_id, amount, coefficient, game_id, event, is_won);
    bet_repo.add(bet).unwrap();

    let bet_id = bet_repo.next_id();
    let amount = Amount::new(300, Some(MIN_BET_AMOUNT)).unwrap();
    let coefficient = (2.50).try_into().unwrap();
    let event = Event::WDL(Winner::W1);
    let is_won = Some(false);
    let bet = Bet::new(bet_id, sim_id, amount, coefficient, game_id, event, is_won);
    bet_repo.add(bet).unwrap();

    let res = bet_repo.min_coefficient_lose();

    assert_eq!(res, Some((2.30).try_into().unwrap()));

    sim_repo.remove_by_id(sim_id);
}

#[test]
fn not_calculated_bets() {
    let pool = init_pool();

    let mut bet_repo = BetRepo::new(pool.clone());
    let mut sim_repo = SimulationRepo::new(pool.clone());
    let sim_id = sim_repo.next_id();
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let balance = Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let simulation = Simulation::new(sim_id, ip, balance);
    sim_repo.add(simulation).unwrap();
    let mut game_repo = GameRepo::new(pool.clone());
    let game_id = game_repo.next_id();
    let mut team_repo = TeamRepo::new(pool);
    let team_ids = team_repo.all_teams_id();
    let game = Game::new(game_id, sim_id, team_ids[0], team_ids[1], 1);
    game_repo.add(game).unwrap();

    let bet_id = bet_repo.next_id();
    let amount = Amount::new(100, Some(MIN_BET_AMOUNT)).unwrap();
    let coefficient = (2.40).try_into().unwrap();
    let event = Event::WDL(Winner::W1);
    let is_won = None;
    let bet1 = Bet::new(bet_id, sim_id, amount, coefficient, game_id, event, is_won);
    bet_repo.add(bet1).unwrap();

    let bet_id = bet_repo.next_id();
    let amount = Amount::new(200, Some(MIN_BET_AMOUNT)).unwrap();
    let coefficient = (2.30).try_into().unwrap();
    let event = Event::WDL(Winner::W1);
    let is_won = Some(false);
    let bet2 = Bet::new(bet_id, sim_id, amount, coefficient, game_id, event, is_won);
    bet_repo.add(bet2).unwrap();

    let bet_id = bet_repo.next_id();
    let amount = Amount::new(300, Some(MIN_BET_AMOUNT)).unwrap();
    let coefficient = (2.50).try_into().unwrap();
    let event = Event::WDL(Winner::W1);
    let is_won = None;
    let bet3 = Bet::new(bet_id, sim_id, amount, coefficient, game_id, event, is_won);
    bet_repo.add(bet3).unwrap();

    let res = bet_repo.not_calculated_bets();

    assert_eq!(res.len(), 2);

    sim_repo.remove_by_id(sim_id);
}

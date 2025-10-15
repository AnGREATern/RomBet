use anyhow::Result;
use std::net::{IpAddr, Ipv4Addr};

use application::{
    config::SetupConfig,
    repository::{IGameRepo, ISimulationRepo},
    service::SimulationService,
    usecase::{CreateRound, Start},
};
use db::{
    init_pool,
    repository::{GameRepo, GameStatRepo, SimulationRepo, TeamRepo},
};
use domain::{
    entity::Simulation,
    value_object::{Amount, MIN_BALANCE_AMOUNT},
};

#[test]
fn test_simulation_service_start_new_simulation() -> Result<()> {
    let pool = init_pool();
    let service_game_repo = GameRepo::new(pool.clone());
    let service_team_repo = TeamRepo::new(pool.clone());
    let service_simulation_repo = SimulationRepo::new(pool.clone());
    let service_game_stat_repo = GameStatRepo::new(pool.clone());
    let config = SetupConfig {
        balance: Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap(),
    };
    let service = SimulationService::new(
        service_game_repo,
        service_team_repo,
        service_game_stat_repo,
        service_simulation_repo,
        config,
    );
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 10));

    let simulation = service.start(ip)?;

    assert_eq!(simulation.ip(), ip);
    assert_eq!(simulation.balance().clear_value(), 1000);
    assert_eq!(simulation.round(), 0);

    Ok(())
}

#[test]
fn test_simulation_service_start_existing_simulation() -> Result<()> {
    let pool = init_pool();
    let service_game_repo = GameRepo::new(pool.clone());
    let service_team_repo = TeamRepo::new(pool.clone());
    let service_simulation_repo = SimulationRepo::new(pool.clone());
    let service_game_stat_repo = GameStatRepo::new(pool.clone());
    let config = SetupConfig {
        balance: Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap(),
    };
    let service = SimulationService::new(
        service_game_repo,
        service_team_repo,
        service_game_stat_repo,
        service_simulation_repo,
        config,
    );
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 11));
    let cleanup_pool = init_pool();
    let setup_simulation_repo = SimulationRepo::new(cleanup_pool.clone());
    let id = setup_simulation_repo.next_id();
    let initial_simulation = Simulation::new(id, ip, config.balance, None);
    setup_simulation_repo.add(initial_simulation)?;

    let simulation = service.start(ip)?;

    assert_eq!(simulation.ip(), ip);
    assert_eq!(simulation.id().value(), id.value());

    Ok(())
}

#[test]
fn test_simulation_service_restart_simulation() -> Result<()> {
    let pool = init_pool();
    let service_game_repo = GameRepo::new(pool.clone());
    let service_team_repo = TeamRepo::new(pool.clone());
    let service_simulation_repo = SimulationRepo::new(pool.clone());
    let service_game_stat_repo = GameStatRepo::new(pool.clone());
    let initial_balance = Amount::new(500, Some(MIN_BALANCE_AMOUNT)).unwrap();
    let config = SetupConfig {
        balance: Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap(),
    };
    let service = SimulationService::new(
        service_game_repo,
        service_team_repo,
        service_game_stat_repo,
        service_simulation_repo,
        config,
    );
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 12));
    let cleanup_pool = init_pool();
    let setup_simulation_repo = SimulationRepo::new(cleanup_pool.clone());
    let id = setup_simulation_repo.next_id();
    let mut initial_simulation = Simulation::new(id, ip, initial_balance, None);
    initial_simulation.increment_round();
    setup_simulation_repo.add(initial_simulation)?;

    let simulation = service.restart(id)?;

    assert_eq!(simulation.ip(), ip);
    assert_eq!(simulation.id().value(), id.value());
    assert_eq!(simulation.balance().clear_value(), 1000);
    assert_eq!(simulation.round(), 0);

    Ok(())
}

#[test]
fn test_simulation_service_create_round() -> Result<()> {
    let pool = init_pool();
    let service_game_repo = GameRepo::new(pool.clone());
    let service_team_repo = TeamRepo::new(pool.clone());
    let service_simulation_repo = SimulationRepo::new(pool.clone());
    let service_game_stat_repo = GameStatRepo::new(pool.clone());
    let config = SetupConfig {
        balance: Amount::new(1000, Some(MIN_BALANCE_AMOUNT)).unwrap(),
    };
    let service = SimulationService::new(
        service_game_repo,
        service_team_repo,
        service_game_stat_repo,
        service_simulation_repo,
        config,
    );
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 13));
    let mut simulation = service.start(ip)?;

    let games = service.create_round(&mut simulation)?;

    assert!(!games.is_empty());
    assert_eq!(simulation.round(), 1);
    let verification_pool = init_pool();
    let verification_game_repo = GameRepo::new(verification_pool);
    let game_ids_from_db = verification_game_repo.games_id_by_round(1, simulation.id())?;
    assert_eq!(games.len(), game_ids_from_db.len());

    Ok(())
}

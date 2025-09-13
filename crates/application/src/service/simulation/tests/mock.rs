use std::net::Ipv4Addr;

use anyhow::anyhow;
use domain::value_object::Amount;
use uuid::Uuid;

use super::super::*;
use crate::repository::{MockIGameRepo, MockIGameStatRepo, MockISimulationRepo, MockITeamRepo};

#[test]
fn start_success() {
    let game_repo = MockIGameRepo::new();
    let gs_repo = MockIGameStatRepo::new();
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo.expect_simulation_by_ip().returning(|_| {
        Some(Simulation::new(
            Uuid::now_v7().into(),
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            Amount::new(1000, None).unwrap(),
            None,
        ))
    });
    let team_repo = MockITeamRepo::new();
    let config = SetupConfig {
        balance: Amount::new(10000, None).unwrap(),
    };
    let ss = SimulationService::new(game_repo, team_repo, gs_repo, sim_repo, config);

    let res = ss.start(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST));

    assert!(res.is_ok());
}

#[test]
fn start_failure() {
    let game_repo = MockIGameRepo::new();
    let gs_repo = MockIGameStatRepo::new();
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo.expect_simulation_by_ip().returning(|_| None);
    sim_repo
        .expect_next_id()
        .returning(|| <Id<Simulation>>::from(Uuid::now_v7()));
    sim_repo
        .expect_add()
        .returning(|_| Err(anyhow!("unable to add")));
    let team_repo = MockITeamRepo::new();
    let config = SetupConfig {
        balance: Amount::new(10000, None).unwrap(),
    };
    let ss = SimulationService::new(game_repo, team_repo, gs_repo, sim_repo, config);

    let res = ss.start(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST));

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "unable to add");
}

#[test]
fn restart_success() {
    let game_repo = MockIGameRepo::new();
    let gs_repo = MockIGameStatRepo::new();
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo.expect_update_by_id().returning(|_| Ok(()));
    sim_repo.expect_simulation_by_id().returning(|_| {
        Ok(Simulation::new(
            Uuid::now_v7().into(),
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            Amount::new(1000, None).unwrap(),
            None,
        ))
    });
    let team_repo = MockITeamRepo::new();
    let config = SetupConfig {
        balance: Amount::new(10000, None).unwrap(),
    };
    let ss = SimulationService::new(game_repo, team_repo, gs_repo, sim_repo, config);

    let res = ss.restart(Uuid::now_v7().into());

    assert!(res.is_ok());
}

#[test]
fn restart_failure() {
    let game_repo = MockIGameRepo::new();
    let gs_repo = MockIGameStatRepo::new();
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo
        .expect_update_by_id()
        .returning(|_| Err(anyhow!("unable to update")));
    sim_repo.expect_simulation_by_id().returning(|_| {
        Ok(Simulation::new(
            Uuid::now_v7().into(),
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            Amount::new(1000, None).unwrap(),
            None,
        ))
    });
    let team_repo = MockITeamRepo::new();
    let config = SetupConfig {
        balance: Amount::new(10000, None).unwrap(),
    };
    let ss = SimulationService::new(game_repo, team_repo, gs_repo, sim_repo, config);

    let res = ss.restart(Uuid::now_v7().into());

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "unable to update");
}

#[test]
fn create_round_success() {
    let mut game_repo = MockIGameRepo::new();
    game_repo
        .expect_games_id_by_round()
        .returning(|_, _| Ok(vec![]));
    game_repo
        .expect_next_id()
        .returning(|| <Id<Game>>::from(Uuid::now_v7()));
    game_repo.expect_add().returning(|_| Ok(()));
    let gs_repo = MockIGameStatRepo::new();
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo.expect_update_by_id().returning(|_| Ok(()));
    let mut team_repo = MockITeamRepo::new();
    team_repo.expect_all_teams_id().returning(|| {
        vec![
            <Id<Team>>::from(Uuid::now_v7()),
            <Id<Team>>::from(Uuid::now_v7()),
        ]
    });
    team_repo
        .expect_team_by_id()
        .returning(|_| Ok(Team::new(<Id<Team>>::from(Uuid::now_v7()), "CSKA".into())));
    let config = SetupConfig {
        balance: Amount::new(10000, None).unwrap(),
    };
    let ss = SimulationService::new(game_repo, team_repo, gs_repo, sim_repo, config);
    let mut simulation = Simulation::new(
        Uuid::now_v7().into(),
        std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
        Amount::new(1000, None).unwrap(),
        None,
    );

    let res = ss.create_round(&mut simulation);

    assert!(res.is_ok());
}

#[test]
fn create_round_failure() {
    let mut game_repo = MockIGameRepo::new();
    game_repo
        .expect_games_id_by_round()
        .returning(|_, _| Ok(vec![]));
    game_repo
        .expect_next_id()
        .returning(|| <Id<Game>>::from(Uuid::now_v7()));
    game_repo
        .expect_add()
        .returning(|_| Err(anyhow!("unable to add")));
    let gs_repo = MockIGameStatRepo::new();
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo.expect_update_by_id().returning(|_| Ok(()));
    let mut team_repo = MockITeamRepo::new();
    team_repo.expect_all_teams_id().returning(|| {
        vec![
            <Id<Team>>::from(Uuid::now_v7()),
            <Id<Team>>::from(Uuid::now_v7()),
        ]
    });
    team_repo
        .expect_team_by_id()
        .returning(|_| Ok(Team::new(<Id<Team>>::from(Uuid::now_v7()), "CSKA".into())));
    let config = SetupConfig {
        balance: Amount::new(10000, None).unwrap(),
    };
    let ss = SimulationService::new(game_repo, team_repo, gs_repo, sim_repo, config);
    let mut simulation = Simulation::new(
        Uuid::now_v7().into(),
        std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
        Amount::new(1000, None).unwrap(),
        None,
    );

    let res = ss.create_round(&mut simulation);

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "unable to add");
}

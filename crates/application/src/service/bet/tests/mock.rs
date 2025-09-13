use anyhow::anyhow;
use std::net::Ipv4Addr;
use uuid::Uuid;

use super::super::*;
use crate::repository::{MockIBetRepo, MockIGameRepo, MockIGameStatRepo, MockISimulationRepo};

#[test]
fn make_bet_success() {
    let mut bet_repo = MockIBetRepo::new();
    bet_repo
        .expect_next_id()
        .returning(|| <Id<Bet>>::from(Uuid::now_v7()));
    bet_repo.expect_add().returning(|_| Ok(()));
    let game_repo = MockIGameRepo::new();
    let gs_repo = MockIGameStatRepo::new();
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo.expect_simulation_by_id().returning(|_| {
        Ok(Simulation::new(
            Uuid::now_v7().into(),
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            Amount::new(1000, None).unwrap(),
            None,
        ))
    });
    sim_repo.expect_update_by_id().returning(|_| Ok(()));
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let bs = BetService::new(bet_repo, game_repo, gs_repo, sim_repo, config);
    let game = Game::new(
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        1,
    );
    let amount = Amount::new(1000, None).unwrap();
    let event = Event::WDL(Winner::W1);
    let coefficient = 189.try_into().unwrap();

    let res = bs.make_bet(&game, amount, event, coefficient);

    assert!(res.is_ok());
}

#[test]
fn make_bet_failure() {
    let mut bet_repo = MockIBetRepo::new();
    bet_repo
        .expect_next_id()
        .returning(|| <Id<Bet>>::from(Uuid::now_v7()));
    bet_repo.expect_add().returning(|_| Err(anyhow!("err")));
    let game_repo = MockIGameRepo::new();
    let gs_repo = MockIGameStatRepo::new();
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo.expect_simulation_by_id().returning(|_| {
        Ok(Simulation::new(
            Uuid::now_v7().into(),
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            Amount::new(1000, None).unwrap(),
            None,
        ))
    });
    sim_repo.expect_update_by_id().returning(|_| Ok(()));
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let bs = BetService::new(bet_repo, game_repo, gs_repo, sim_repo, config);
    let game = Game::new(
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        1,
    );
    let amount = Amount::new(1000, None).unwrap();
    let event = Event::WDL(Winner::W1);
    let coefficient = 189.try_into().unwrap();

    let res = bs.make_bet(&game, amount, event, coefficient);

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "err");
}

#[test]
fn calculate_coefficients_success() {
    let bet_repo = MockIBetRepo::new();
    let mut game_repo = MockIGameRepo::new();
    game_repo
        .expect_games_id_by_team_id()
        .returning(|_, _, _| Ok(vec![(<Id<Game>>::from(Uuid::now_v7()), true)]));
    game_repo
        .expect_h2hs_id_by_team_id()
        .returning(|_, _, _, _| Ok(vec![(<Id<Game>>::from(Uuid::now_v7()), true)]));
    let mut gs_repo = MockIGameStatRepo::new();
    gs_repo
        .expect_winner_by_game_id()
        .returning(|_, _| Some(Winner::X));
    gs_repo
        .expect_score_by_game_id()
        .returning(|_, _| Some((2u8, 0u8)));
    let sim_repo = MockISimulationRepo::new();
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let bs = BetService::new(bet_repo, game_repo, gs_repo, sim_repo, config);
    let game = Game::new(
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        1,
    );

    let res = bs.calculate_coefficients(&game);

    assert!(res.is_ok());
}

#[test]
fn calculate_coefficients_failure() {
    let bet_repo = MockIBetRepo::new();
    let mut game_repo = MockIGameRepo::new();
    game_repo
        .expect_games_id_by_team_id()
        .returning(|_, _, _| Ok(vec![(<Id<Game>>::from(Uuid::now_v7()), true)]));
    game_repo
        .expect_h2hs_id_by_team_id()
        .returning(|_, _, _, _| Err(anyhow!("err")));
    let mut gs_repo = MockIGameStatRepo::new();
    gs_repo
        .expect_winner_by_game_id()
        .returning(|_, _| Some(Winner::X));
    gs_repo
        .expect_score_by_game_id()
        .returning(|_, _| Some((2u8, 0u8)));
    let sim_repo = MockISimulationRepo::new();
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let bs = BetService::new(bet_repo, game_repo, gs_repo, sim_repo, config);
    let game = Game::new(
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        1,
    );

    let res = bs.calculate_coefficients(&game);

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "err");
}

#[test]
fn calculate_bets_success() {
    let mut bet_repo = MockIBetRepo::new();
    bet_repo.expect_not_calculated_bets().returning(|| {
        vec![Bet::new(
            Uuid::now_v7().into(),
            Uuid::now_v7().into(),
            Amount::new(1000, None).unwrap(),
            189.try_into().unwrap(),
            Uuid::now_v7().into(),
            Event::WDL(Winner::X),
            None,
        )]
    });
    bet_repo.expect_update_status().returning(|_| Ok(()));
    let game_repo = MockIGameRepo::new();
    let mut gs_repo = MockIGameStatRepo::new();
    gs_repo
        .expect_winner_by_game_id()
        .returning(|_, _| Some(Winner::X));
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo.expect_simulation_by_id().returning(|_| {
        Ok(Simulation::new(
            Uuid::now_v7().into(),
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            Amount::new(1000, None).unwrap(),
            None,
        ))
    });
    sim_repo.expect_update_by_id().returning(|_| Ok(()));
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let bs = BetService::new(bet_repo, game_repo, gs_repo, sim_repo, config);

    let res = bs.calculate_bets();

    assert!(res.is_ok());
}

#[test]
fn calculate_bets_failure() {
    let mut bet_repo = MockIBetRepo::new();
    bet_repo.expect_not_calculated_bets().returning(|| {
        vec![Bet::new(
            Uuid::now_v7().into(),
            Uuid::now_v7().into(),
            Amount::new(1000, None).unwrap(),
            189.try_into().unwrap(),
            Uuid::now_v7().into(),
            Event::WDL(Winner::X),
            None,
        )]
    });
    bet_repo.expect_update_status().returning(|_| Ok(()));
    let game_repo = MockIGameRepo::new();
    let mut gs_repo = MockIGameStatRepo::new();
    gs_repo
        .expect_winner_by_game_id()
        .returning(|_, _| Some(Winner::X));
    let mut sim_repo = MockISimulationRepo::new();
    sim_repo.expect_simulation_by_id().returning(|_| {
        Ok(Simulation::new(
            Uuid::now_v7().into(),
            std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
            Amount::new(1000, None).unwrap(),
            None,
        ))
    });
    sim_repo
        .expect_update_by_id()
        .returning(|_| Err(anyhow!("err")));
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let bs = BetService::new(bet_repo, game_repo, gs_repo, sim_repo, config);

    let res = bs.calculate_bets();

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "err");
}

#[test]
fn make_report_full() {
    let mut bet_repo = MockIBetRepo::new();
    let coefficient = Coefficient::try_from(189).unwrap();
    bet_repo
        .expect_min_coefficient_lose()
        .returning(move || Some(coefficient.clone()));
    let game_repo = MockIGameRepo::new();
    let gs_repo = MockIGameStatRepo::new();
    let sim_repo = MockISimulationRepo::new();
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let bs = BetService::new(bet_repo, game_repo, gs_repo, sim_repo, config);
    let amount = Amount::new(1000, None).unwrap();

    let bs = bs.make_report(amount);

    assert!(bs.start_balance() == amount);
    assert_eq!(bs.min_coefficient_lose(), Some(coefficient));
}

#[test]
fn make_report_empty() {
    let mut bet_repo = MockIBetRepo::new();
    bet_repo.expect_min_coefficient_lose().returning(|| None);
    let game_repo = MockIGameRepo::new();
    let gs_repo = MockIGameStatRepo::new();
    let sim_repo = MockISimulationRepo::new();
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let bs = BetService::new(bet_repo, game_repo, gs_repo, sim_repo, config);
    let amount = Amount::new(1000, None).unwrap();

    let bs = bs.make_report(amount);

    assert!(bs.start_balance() == amount);
    assert_eq!(bs.min_coefficient_lose(), None);
}

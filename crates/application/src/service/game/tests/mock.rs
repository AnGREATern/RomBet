use std::net::Ipv4Addr;

use anyhow::anyhow;
use domain::value_object::{Amount, Margin};
use uuid::Uuid;

use super::super::*;
use crate::repository::{MockIGameRepo, MockIGameStatRepo, MockITeamRepo};

#[test]
fn randomize_game_success() {
    let game = Game::new(
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        1,
    );
    let mut game_repo = MockIGameRepo::new();
    game_repo
        .expect_games_id_by_team_id()
        .returning(|_, _, _| Ok(vec![(<Id<Game>>::from(Uuid::now_v7()), true)]));
    game_repo
        .expect_h2hs_id_by_team_id()
        .returning(|_, _, _, _| Ok(vec![(<Id<Game>>::from(Uuid::now_v7()), true)]));
    game_repo
        .expect_game_by_id()
        .returning(move |_| Ok(game.clone()));
    let mut gs_repo = MockIGameStatRepo::new();
    gs_repo
        .expect_winner_by_game_id()
        .returning(|_, _| Some(Winner::X));
    gs_repo
        .expect_score_by_game_id()
        .returning(|_, _| Some((2u8, 2u8)));
    gs_repo
        .expect_goals_by_game_id()
        .returning(|_, _| Some(2u8));
    gs_repo
        .expect_next_id()
        .returning(|| <Id<GameStat>>::from(Uuid::now_v7()));
    gs_repo.expect_add().returning(|_| Ok(()));
    let mut team_repo = MockITeamRepo::new();
    team_repo
        .expect_team_by_id()
        .returning(|_| Ok(Team::new(<Id<Team>>::from(Uuid::now_v7()), "CSKA".into())));
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let gs = GameService::new(game_repo, gs_repo, team_repo, config);

    let res = gs.randomize_game(&game);

    assert!(res.is_ok());
}

#[test]
fn randomize_game_failure() {
    let game = Game::new(
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        Uuid::now_v7().into(),
        1,
    );
    let mut game_repo = MockIGameRepo::new();
    game_repo
        .expect_games_id_by_team_id()
        .returning(|_, _, _| Ok(vec![(<Id<Game>>::from(Uuid::now_v7()), true)]));
    game_repo
        .expect_h2hs_id_by_team_id()
        .returning(|_, _, _, _| Ok(vec![(<Id<Game>>::from(Uuid::now_v7()), true)]));
    game_repo
        .expect_game_by_id()
        .returning(move |_| Ok(game.clone()));
    let mut gs_repo = MockIGameStatRepo::new();
    gs_repo
        .expect_winner_by_game_id()
        .returning(|_, _| Some(Winner::X));
    gs_repo
        .expect_score_by_game_id()
        .returning(|_, _| Some((2u8, 2u8)));
    gs_repo
        .expect_goals_by_game_id()
        .returning(|_, _| Some(2u8));
    gs_repo
        .expect_next_id()
        .returning(|| <Id<GameStat>>::from(Uuid::now_v7()));
    gs_repo.expect_add().returning(|_| Err(anyhow!("smt")));
    let mut team_repo = MockITeamRepo::new();
    team_repo
        .expect_team_by_id()
        .returning(|_| Ok(Team::new(<Id<Team>>::from(Uuid::now_v7()), "CSKA".into())));
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let gs = GameService::new(game_repo, gs_repo, team_repo, config);

    let res = gs.randomize_game(&game);

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "smt");
}

#[test]
fn randomize_round_success() {
    let mut game_repo = MockIGameRepo::new();
    game_repo
        .expect_games_id_by_round()
        .returning(|_, _| Ok(vec![]));
    let mut gs_repo = MockIGameStatRepo::new();
    gs_repo.expect_game_stat_by_game_id().returning(|_| {
        Ok(GameStat::new(
            <Id<GameStat>>::from(Uuid::now_v7()),
            <Id<Game>>::from(Uuid::now_v7()),
            2,
            1,
        ))
    });
    let team_repo = MockITeamRepo::new();
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let gs = GameService::new(game_repo, gs_repo, team_repo, config);
    let simulation = Simulation::new(
        Uuid::now_v7().into(),
        std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
        Amount::new(1000, None).unwrap(),
        None,
    );

    let res = gs.randomize_round(&simulation);

    assert!(res.is_ok());
}

#[test]
fn randomize_round_failure() {
    let mut game_repo = MockIGameRepo::new();
    game_repo
        .expect_games_id_by_round()
        .returning(|_, _| Ok(vec![<Id<Game>>::from(Uuid::now_v7())]));
    let mut gs_repo = MockIGameStatRepo::new();
    gs_repo.expect_game_stat_by_game_id().returning(|_| {
        Ok(GameStat::new(
            <Id<GameStat>>::from(Uuid::now_v7()),
            <Id<Game>>::from(Uuid::now_v7()),
            2,
            1,
        ))
    });
    let team_repo = MockITeamRepo::new();
    let config = CoefficientConfig {
        tracked_games: 5,
        margin: Margin::try_from(0.12).unwrap(),
        alpha: 15,
        totals: vec![2, 3],
        deviation_min: 0.8,
        deviation_max: 1.2,
    };
    let gs = GameService::new(game_repo, gs_repo, team_repo, config);
    let simulation = Simulation::new(
        Uuid::now_v7().into(),
        std::net::IpAddr::V4(Ipv4Addr::LOCALHOST),
        Amount::new(1000, None).unwrap(),
        None,
    );

    let res = gs.randomize_round(&simulation);

    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        "Last round already randomized"
    );
}

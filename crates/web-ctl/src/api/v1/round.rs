use anyhow::Result;
use axum::Json;
use axum::extract::{ConnectInfo, State};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::error::FailureResponse;
use crate::state::AppState;
use application::service::{DisplayedGame, DisplayedGameStat};
use application::usecase::{CalculateBet, CreateRound, RandomizeRound};

#[derive(Serialize)]
pub struct CreateRoundSuccessResponse {
    pub round: u32,
    pub games: Vec<DisplayedGame>,
}

pub async fn create_round(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<CreateRoundSuccessResponse>, FailureResponse> {
    debug!("Perform create_round operation");
    let mut simulation = state
        .simulation(addr.ip())
        .map_err(|e| FailureResponse::not_found(e, "SIMULATION_NOT_FOUND"))?;
    let sim_service = state.simulation_service();

    let games: Vec<DisplayedGame> = sim_service.create_round(&mut simulation).map_err(|e| {
        warn!(?e);
        FailureResponse::unprocessable_entity(
            "Unable to create new round while last didn't finished",
            "UNABLE_CREATE_ROUND",
        )
    })?;
    let round = simulation.round();
    info!(round, "Games created");

    Ok(CreateRoundSuccessResponse { round, games }.into())
}

#[derive(Serialize)]
pub struct RandomizeRoundSuccessResponse {
    pub round: u32,
    pub games_stat: Vec<DisplayedGameStat>,
    pub profit: f64,
}

pub async fn randomize_round(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<RandomizeRoundSuccessResponse>, FailureResponse> {
    debug!("Perform randomize round operation");
    let simulation = state
        .simulation(addr.ip())
        .map_err(|e| FailureResponse::not_found(e, "SIMULATION_NOT_FOUND"))?;
    let game_service = state.game_service();
    let bet_service = state.bet_service();
    let games_stat = game_service.randomize_round(&simulation).map_err(|e| {
        warn!(?e);
        FailureResponse::unprocessable_entity(
            "Last round already randomized or doesn't exists",
            "UNABLE_RANDOMIZE_ROUND",
        )
    })?;

    let round = simulation.round();
    info!(round, "Show game results");

    let profit = f64::from(bet_service.calculate_bets()?);
    info!(profit, "Credit to balance");

    Ok(RandomizeRoundSuccessResponse {
        round,
        games_stat,
        profit,
    }
    .into())
}

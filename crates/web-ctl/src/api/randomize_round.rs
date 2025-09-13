use anyhow::Result;
use axum::Json;
use axum::extract::{ConnectInfo, State};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info};

use crate::error::FailureResponse;
use crate::state::AppState;
use application::service::DisplayedGameStat;
use application::usecase::{CalculateBet, RandomizeRound};

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
    let simulation = state.simulation(addr.ip())?;
    let game_service = state.game_service();
    let bet_service = state.bet_service();
    let games_stat = game_service.randomize_round(&simulation)?;

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

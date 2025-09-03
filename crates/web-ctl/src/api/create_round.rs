use anyhow::Result;
use axum::Json;
use axum::extract::{ConnectInfo, State};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info};

use crate::error::FailureResponse;
use crate::state::AppState;
use application::service::DisplayedGame;
use application::usecase::CreateRound;

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
    let mut simulation = state.simulation(addr.ip())?;
    let sim_service = state.simulation_service();

    let games: Vec<DisplayedGame> = sim_service.create_round(&mut simulation)?;
    let round = simulation.round();
    info!(round, "Games created");

    Ok(CreateRoundSuccessResponse { round, games }.into())
}

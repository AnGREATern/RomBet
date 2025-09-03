use axum::Json;
use axum::extract::{ConnectInfo, State};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info};

use crate::error::FailureResponse;
use crate::state::AppState;
use application::usecase::Start;

#[derive(Serialize)]
pub struct StartSuccessResponse {
    pub balance: f64,
}

pub async fn start(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<StartSuccessResponse>, FailureResponse> {
    let sim_service = state.simulation_service();
    let simulation = sim_service.start(addr.ip())?;
    let balance = f64::from(simulation.balance());
    info!(balance, "Simulation started successfully");

    Ok(StartSuccessResponse { balance }.into())
}

pub async fn restart(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<StartSuccessResponse>, FailureResponse> {
    debug!("Perform restart operation");
    let sim_service = state.simulation_service();
    let simulation = state.simulation(addr.ip())?;
    sim_service.restart(simulation.id())?;
    let balance = f64::from(simulation.balance());
    info!(balance, "Restart successful");

    Ok(StartSuccessResponse { balance }.into())
}

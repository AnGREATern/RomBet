use axum::Json;
use axum::extract::{ConnectInfo, State};
use domain::value_object::BetStatistics;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info};

use crate::error::FailureResponse;
use crate::state::AppState;
use application::usecase::{MakeReport, Start};

#[derive(Serialize)]
pub struct StartSuccessResponse {
    pub id: String,
    pub balance: f64,
}

pub async fn start(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<StartSuccessResponse>, FailureResponse> {
    let sim_service = state.simulation_service();
    let simulation = sim_service.start(addr.ip())?;
    let balance = f64::from(simulation.balance());
    let id = simulation.id().value().to_string();
    info!(balance, "Simulation started successfully");

    Ok(StartSuccessResponse { id, balance }.into())
}

pub async fn restart(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<StartSuccessResponse>, FailureResponse> {
    debug!("Perform restart operation");
    let sim_service = state.simulation_service();
    let simulation = state
        .simulation(addr.ip())
        .map_err(|e| FailureResponse::not_found(e, "SIMULATION_NOT_FOUND"))?;
    let simulation = sim_service
        .restart(simulation.id())
        .map_err(|e| FailureResponse::not_found(e, "SIMULATION_NOT_FOUND"))?;
    let balance = f64::from(simulation.balance());
    let id = simulation.id().value().to_string();
    info!(balance, "Restart successful");

    Ok(StartSuccessResponse { id, balance }.into())
}

#[derive(Serialize)]
pub struct BalanceSuccessResponse {
    pub amount: f64,
}

pub async fn balance(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<BalanceSuccessResponse>, FailureResponse> {
    debug!("Perform balance operation");
    let simulation = state
        .simulation(addr.ip())
        .map_err(|e| FailureResponse::not_found(e, "SIMULATION_NOT_FOUND"))?;
    let amount = simulation.balance().into();

    Ok(BalanceSuccessResponse { amount }.into())
}

#[derive(Serialize)]
pub struct MakeReportSuccessResponse {
    pub stat: BetStatistics,
}

pub async fn report(
    State(state): State<Arc<AppState>>,
) -> Result<Json<MakeReportSuccessResponse>, FailureResponse> {
    debug!("Perform make report operation");
    let bet_service = state.bet_service();
    let stat = bet_service.make_report(state.setup_config().balance);

    Ok(MakeReportSuccessResponse { stat }.into())
}

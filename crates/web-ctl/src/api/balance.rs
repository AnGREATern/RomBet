use anyhow::Result;
use axum::Json;
use axum::extract::{ConnectInfo, State};
use domain::value_object::Amount;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::debug;

use crate::error::FailureResponse;
use crate::state::AppState;

#[derive(Serialize)]
pub struct BalanceSuccessResponse {
    pub amount: Amount,
}

pub async fn balance(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<BalanceSuccessResponse>, FailureResponse> {
    debug!("Perform balance operation");
    let simulation = state.simulation(addr.ip())?;
    let amount = simulation.balance();

    Ok(BalanceSuccessResponse { amount }.into())
}

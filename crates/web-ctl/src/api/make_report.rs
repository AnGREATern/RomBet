use anyhow::Result;
use application::usecase::MakeReport;
use axum::Json;
use axum::extract::State;
use domain::value_object::BetStatistics;
use serde::Serialize;
use std::sync::Arc;
use tracing::debug;

use crate::error::FailureResponse;
use crate::state::AppState;

#[derive(Serialize)]
pub struct MakeReportSuccessResponse {
    pub stat: BetStatistics,
}

pub async fn make_report(
    State(state): State<Arc<AppState>>,
) -> Result<Json<MakeReportSuccessResponse>, FailureResponse> {
    debug!("Perform make report operation");
    let bet_service = state.bet_service();
    let stat = bet_service.make_report(state.setup_config().balance);

    Ok(MakeReportSuccessResponse { stat }.into())
}

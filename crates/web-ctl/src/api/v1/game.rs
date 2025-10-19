use anyhow::Result;
use application::repository::IGameRepo;
use application::usecase::MakeBet;
use axum::http::StatusCode;
use axum::Json;
use axum::extract::{Path, State};
use domain::entity::Game;
use domain::value_object::{Amount, Coefficient, Event, MIN_BALANCE_AMOUNT};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info};
use uuid::Uuid;

use crate::error::FailureResponse;
use crate::state::AppState;

#[derive(Serialize)]
pub struct CalculateCoefficientsSuccessResponse {
    pub events: Vec<Event>,
    pub coefficients: Vec<Coefficient>,
}

pub async fn calculate_coefficients(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<CalculateCoefficientsSuccessResponse>, FailureResponse> {
    debug!("Perform calculate coefficients operation");
    let game_repo = state.game_repo();
    let game = game_repo
        .game_by_id(id.into())
        .map_err(|e| FailureResponse::not_found(e, "GAME_NOT_FOUND"))?;
    info!("Game selected");
    let bet_service = state.bet_service();
    let (events, coefficients) = bet_service
        .calculate_coefficients(&game)?
        .into_iter()
        .unzip();

    Ok(CalculateCoefficientsSuccessResponse {
        events,
        coefficients,
    }
    .into())
}

#[derive(Deserialize)]
pub struct MakeBetRequest {
    pub event: Event,
    pub coefficient: Coefficient,
    pub value: f64,
}

pub async fn make_bet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<MakeBetRequest>,
) -> Result<StatusCode, FailureResponse> {
    debug!("Perform make bet operation");
    let game_repo = state.game_repo();
    let game = game_repo
        .game_by_id(id.into())
        .map_err(|e| FailureResponse::not_found(e, "GAME_NOT_FOUND"))?;
    info!("Game selected");
    let amount = Amount::new_with_casting(req.value, Some(MIN_BALANCE_AMOUNT))?;
    debug!("Bet amount parsed");
    info!("Bet made");
    let bet_service = state.bet_service();
    bet_service.make_bet(&game, amount, req.event, req.coefficient)?;

    Ok(StatusCode::CREATED)
}

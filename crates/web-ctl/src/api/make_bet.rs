use anyhow::Result;
use application::usecase::MakeBet;
use axum::Json;
use axum::extract::{ConnectInfo, State};
use domain::entity::{Game, Team};
use domain::value_object::{Amount, Coefficient, Event, Id, MIN_BALANCE_AMOUNT};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{debug, info};

use crate::error::FailureResponse;
use crate::state::AppState;

#[derive(Serialize)]
pub struct CalculateCoefficientsSuccessResponse {
    pub events: Vec<Event>,
    pub coefficients: Vec<Coefficient>,
}

#[derive(Deserialize)]
pub struct CalculateCoefficientsRequest {
    pub game_id: Id<Game>,
    pub home_team_id: Id<Team>,
    pub guest_team_id: Id<Team>,
}

pub async fn calculate_coefficients(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<CalculateCoefficientsRequest>,
) -> Result<Json<CalculateCoefficientsSuccessResponse>, FailureResponse> {
    debug!("Perform calculate coefficients operation");
    let simulation = state.simulation(addr.ip())?;
    let game = Game::new(
        req.game_id,
        simulation.id(),
        req.home_team_id,
        req.guest_team_id,
        simulation.round(),
    );
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
    pub game: Game,
    pub event: Event,
    pub coefficient: Coefficient,
    pub value: f64,
}

pub async fn make_bet(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MakeBetRequest>,
) -> Result<(), FailureResponse> {
    debug!("Perform make bet operation");
    let amount = Amount::new_with_casting(req.value, Some(MIN_BALANCE_AMOUNT))?;
    debug!("Bet amount parsed");
    info!("Bet made");
    let bet_service = state.bet_service();
    bet_service.make_bet(&req.game, amount, req.event, req.coefficient)?;

    Ok(())
}

use anyhow::Result;
use application::repository::ITeamRepo;
use application::usecase::CreateTeam;
use axum::Json;
use axum::extract::{ConnectInfo, Path, State};
use domain::entity::Team;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::debug;
use uuid::Uuid;

use crate::error::FailureResponse;
use crate::state::AppState;

#[derive(Serialize)]
pub struct AllTeamsResponseResponse(Vec<Team>);

pub async fn all_teams(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AllTeamsResponseResponse>, FailureResponse> {
    debug!("Get all teams");
    let repo = state.team_repo();
    let teams = repo.all_teams();

    Ok(AllTeamsResponseResponse(teams).into())
}

#[derive(Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
}

pub async fn create_team(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateTeamRequest>,
) -> Result<(), FailureResponse> {
    debug!("Create a new team");
    let team_service = state.team_service();
    team_service.add_team(req.name)?;

    Ok(())
}

#[derive(Serialize)]
pub struct TeamByIDResponse(Team);

pub async fn get_team(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<TeamByIDResponse>, FailureResponse> {
    debug!("Get team by id");
    let repo = state.team_repo();
    let team = repo.team_by_id(id.into())?;

    Ok(TeamByIDResponse(team).into())
}

#[derive(Deserialize)]
pub struct UpdateTeamRequest {
    pub name: String,
}

pub async fn update_team(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTeamRequest>,
) -> Result<Json<TeamByIDResponse>, FailureResponse> {
    debug!("Update team by id");
    let repo = state.team_repo();
    let team = Team::new(id.into(), req.name);
    repo.update(team.clone())?;

    Ok(TeamByIDResponse(team).into())
}

pub async fn delete_team(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<(), FailureResponse> {
    debug!("Delete team by id");
    let repo = state.team_repo();
    repo.delete(id.into())?;

    Ok(())
}


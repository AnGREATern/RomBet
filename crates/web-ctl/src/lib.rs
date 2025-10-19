mod api;
mod error;
mod state;

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use std::path::Path;
use std::{env, sync::Arc};
use tokio::net::TcpListener;
use tracing::info;

use crate::api::{
    balance::balance,
    create_round::create_round,
    make_bet::{calculate_coefficients, make_bet},
    make_report::make_report,
    randomize_round::randomize_round,
    start::{restart, start},
    v1,
    v1::team::{all_teams, create_team, delete_team, get_team, update_team},
};
use infrastructure::{config, logger};
use state::AppState;

#[tokio::main]
pub async fn start_server() -> Result<()> {
    dotenv().ok();
    logger::init_default_logger();
    let config = config::load_from_file(Path::new("config.toml"))?;
    info!("Config applied");
    let app_state = Arc::new(AppState::try_from(config)?);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let old_api_router = Router::new()
        .route("/start", get(start))
        .route("/restart", post(restart))
        .route("/create_round", post(create_round))
        .route("/randomize_round", post(randomize_round))
        .route("/calculate_coefficients", post(calculate_coefficients))
        .route("/make_bet", post(make_bet))
        .route("/make_report", get(make_report))
        .route("/balance", get(balance))
        .with_state(app_state.clone());

    let v1_api_router = Router::new()
        .route("/teams", get(all_teams).post(create_team))
        .route(
            "/teams/{id}",
            get(get_team).put(update_team).delete(delete_team),
        )
        .route(
            "/simulation",
            post(v1::simulation::start).patch(v1::simulation::restart),
        )
        .route("/simulation/balance", get(v1::simulation::balance))
        .route("/simulation/report", get(v1::simulation::report))
        .route("/round", post(v1::round::create_round))
        .route("/round/results", post(v1::round::randomize_round))
        .route(
            "/game/{id}/coefficients",
            get(v1::game::calculate_coefficients),
        )
        .route("/game/{id}/bet", post(v1::game::make_bet))
        .with_state(app_state);

    let app = Router::new()
        .nest("/api", old_api_router)
        .nest("/api/v1", v1_api_router)
        .layer(cors);

    let addr = env::var("ROM_BET_SOCK")?;
    let listener = TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

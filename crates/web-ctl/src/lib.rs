mod api;
mod error;
mod state;

use anyhow::Result;
use axum::{
    Router,
    response::IntoResponse,
    routing::{get, post},
};
use dotenv::dotenv;
use std::net::SocketAddr;
use std::path::Path;
use std::{env, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::api::{
    balance::balance,
    create_round::create_round,
    make_bet::{calculate_coefficients, make_bet},
    make_report::make_report,
    randomize_round::randomize_round,
    start::{restart, start},
};
use infrastructure::{config, logger};
use state::AppState;

async fn spa_handler() -> impl IntoResponse {
    let html = include_str!("../../../frontend/dist/index.html");
    axum::response::Html(html)
}

#[tokio::main]
pub async fn start_server() -> Result<()> {
    dotenv().ok();
    logger::init_default_logger();
    let config = config::load_from_file(Path::new("config.toml"))?;
    info!("Config applied");
    let app_state = Arc::new(AppState::try_from(config)?);

    let api_router = Router::new()
        .route("/api/", post(start))
        .route("/api/restart", post(restart))
        .route("/api/create_round", post(create_round))
        .route("/api/randomize_round", post(randomize_round))
        .route("/api/calculate_coefficients", post(calculate_coefficients))
        .route("/api/make_bet", post(make_bet))
        .route("/api/make_report", get(make_report))
        .route("/api/balance", get(balance))
        .with_state(app_state);

    let app = Router::new()
        .nest("/api", api_router)
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .nest_service("/static", ServeDir::new("frontend/dist/static"))
        .fallback_service(ServeDir::new("frontend/dist").not_found_service(spa_handler()));

    let addr = env::var("ROM_BET_SOCK")?;
    let listener = TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

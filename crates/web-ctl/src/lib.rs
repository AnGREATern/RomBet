mod api;
mod error;
mod state;

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
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

    let app = Router::new()
        .route("/", post(start))
        .route("/restart", post(restart))
        .route("/create_round", post(create_round))
        .route("/randomize_round", post(randomize_round))
        .route("/calculate_coefficients", post(calculate_coefficients))
        .route("/make_bet", post(make_bet))
        .route("/make_report", get(make_report))
        .route("/balance", get(balance))
        .with_state(app_state);

    let addr = env::var("ROM_BET_ADDR")?;
    let listener = TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

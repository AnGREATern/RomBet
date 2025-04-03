use serde::Deserialize;

use domain::value_object::{Amount, Margin};

#[derive(Deserialize)]
pub struct AppConfig {
    pub coefficient: CoefficientConfig,
    pub setup: SetupConfig,
}

#[derive(Deserialize)]
pub struct CoefficientConfig {
    pub tracked_games: u8,
    pub margin: Margin,
    pub alpha: i32,
    pub totals: Vec<u8>,
    pub deviation_min: f64,
    pub deviation_max: f64,
}

#[derive(Deserialize)]
pub struct SetupConfig {
    pub balance: Amount,
}

use serde::Deserialize;
use anyhow::Result;
use std::{fs::File, io::Read, path::Path};

use domain::value_object::Margin;

#[derive(Deserialize)]
pub struct AppConfig {
    pub coefficient: CoefficientConfig,
}

impl AppConfig {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let config = toml::from_str(&buf)?;

        Ok(config)
    }
}

#[derive(Deserialize)]
pub struct CoefficientConfig {
    pub tracked_games: u8,
    pub margin: Margin,
    pub alpha: i32,
}

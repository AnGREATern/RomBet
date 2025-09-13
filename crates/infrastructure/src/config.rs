use anyhow::{Result, anyhow};
use std::{fs::File, io::Read, path::Path};

use application::config::AppConfig;

pub fn load_from_file(path: &Path) -> Result<AppConfig> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    toml::from_str(&buf).map_err(|e| anyhow!("Can't deserialize data: {}", e.message()))
}

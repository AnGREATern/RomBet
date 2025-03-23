use serde::Deserialize;
use anyhow::{Error, Result, bail};

type Float = f64;
const MIN: f64 = 0.0;
const MAX_EXCLUDED: f64 = 1.0;

#[derive(Deserialize)]
pub struct Margin(Float);

impl TryFrom<Float> for Margin {
    type Error = Error;

    fn try_from(value: Float) -> Result<Self> {
        if MIN <= value && value < MAX_EXCLUDED {
            Ok(Margin(value))
        } else {
            bail!("Margin doesn't support this value")
        }
    }
}

impl From<Margin> for Float {
    fn from(value: Margin) -> Self {
        value.0
    }
}

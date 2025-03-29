use anyhow::{Error, Result, bail};
use rand::{Rng, rng};

type Float = f64;
const MIN: f64 = 0.8;
const MAX: f64 = 1.2;

pub struct Deviation(Float);

impl TryFrom<Float> for Deviation {
    type Error = Error;

    fn try_from(value: Float) -> Result<Self> {
        if MIN <= value && value <= MAX {
            Ok(Deviation(value))
        } else {
            bail!("Deviation doesn't support this value")
        }
    }
}

impl From<Deviation> for Float {
    fn from(value: Deviation) -> Self {
        value.0
    }
}

impl Deviation {
    pub fn generate() -> Self {
        Self(rng().random_range(MIN..=MAX))
    }
}

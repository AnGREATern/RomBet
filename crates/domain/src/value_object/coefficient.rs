use anyhow::{Error, Result, bail};
use serde::{Deserialize, Serialize};

type Float = f64;
const MIN_COEFFICIENT: i32 = 1_01;
const PENNY: i32 = 100;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coefficient(i32);

impl Coefficient {
    pub fn clear_value(self) -> i32 {
        self.0
    }
}

impl TryFrom<Float> for Coefficient {
    type Error = Error;

    fn try_from(value: Float) -> Result<Self, Self::Error> {
        let value = (value * PENNY as Float).round() as i32;
        if value > MIN_COEFFICIENT {
            Ok(Coefficient(value))
        } else {
            bail!("Coefficient doesn't support this value")
        }
    }
}

impl TryFrom<i32> for Coefficient {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > MIN_COEFFICIENT {
            Ok(Coefficient(value))
        } else {
            bail!("Coefficient doesn't support this value")
        }
    }
}

impl From<Coefficient> for Float {
    fn from(value: Coefficient) -> Self {
        value.0 as Float / PENNY as Float
    }
}

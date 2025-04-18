use anyhow::{Error, Result, bail};
use serde::Deserialize;

type Float = f64;
const MIN_AMOUNT: i64 = 0_10;
const PENNY: i32 = 100;

#[derive(Deserialize, Copy, Clone, PartialEq, Eq)]
pub struct Amount(i64);

impl Amount {
    pub fn clear_value(self) -> i64 {
        self.0
    }
}

impl TryFrom<Float> for Amount {
    type Error = Error;

    fn try_from(value: Float) -> Result<Self> {
        let value = (value * PENNY as Float).round() as i64;
        if value > MIN_AMOUNT {
            Ok(Amount(value))
        } else {
            bail!("Amount doesn't support this value")
        }
    }
}

impl TryFrom<i64> for Amount {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self> {
        if value > MIN_AMOUNT {
            Ok(Amount(value))
        } else {
            bail!("Amount doesn't support this value")
        }
    }
}

impl From<Amount> for Float {
    fn from(value: Amount) -> Self {
        value.0 as Float / PENNY as Float
    }
}

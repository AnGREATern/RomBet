use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

type Float = f64;
pub const MIN_BET_AMOUNT: i64 = 10_00;
pub const MIN_BALANCE_AMOUNT: i64 = 0;
const PENNY: u8 = 100;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount {
    value: i64,
    l_bound: Option<i64>,
}

impl Amount {
    pub fn clear_value(self) -> i64 {
        self.value
    }
}

impl From<Amount> for Float {
    fn from(amount: Amount) -> Self {
        amount.value as Float / PENNY as Float
    }
}

impl Amount {
    pub fn new(value: i64, l_bound: Option<i64>) -> Result<Self> {
        if l_bound.is_some_and(|b| value < b) {
            bail!("Amount doesn't support this value")
        }
        Ok(Self { value, l_bound })
    }

    pub fn new_with_casting(value: f64, l_bound: Option<i64>) -> Result<Self> {
        let value = (value * PENNY as Float).round() as i64;
        if l_bound.is_some_and(|b| value < b) {
            bail!("Amount doesn't support this value")
        }
        Ok(Self { value, l_bound })
    }
}

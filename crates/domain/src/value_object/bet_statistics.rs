use super::{Amount, Coefficient};

pub struct BetStatistics {
    min_coefficient_lose: Option<Coefficient>,
    start_balance: Amount,
}

impl BetStatistics {
    pub fn new(start_balance: Amount, min_coefficient_lose: Option<Coefficient>) -> Self {
        Self {
            min_coefficient_lose,
            start_balance,
        }
    }
}

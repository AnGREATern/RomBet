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

    pub fn min_coefficient_lose(&self) -> Option<Coefficient> {
        self.min_coefficient_lose
    }

    pub fn start_balance(&self) -> Amount {
        self.start_balance
    }
}

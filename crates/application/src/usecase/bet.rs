use anyhow::Result;

use domain::entity::{Bet, Game};
use domain::value_object::{Amount, BetStatistics, Coefficient, Event};

pub trait MakeBet {
    fn make_bet(
        &self,
        game: &Game,
        amount: Amount,
        event: Event,
        coefficient: Coefficient,
    ) -> Result<()>;

    fn calculate_coefficients(&self, game: &Game) -> Result<Vec<(Event, Coefficient)>>;

    fn calculate_winner_coefficients(&self, game: &Game) -> Result<Vec<(Event, Coefficient)>>;

    fn calculate_total_coefficients(&self, game: &Game) -> Result<Vec<(Event, Coefficient)>>;
}

pub trait CalculateBet {
    fn calculate_bets(&self) -> Result<f64>;

    fn calculate_bet(&self, bet: Bet) -> Result<f64>;
}

pub trait MakeReport {
    fn make_report(&self, start_balance: Amount) -> BetStatistics;
}

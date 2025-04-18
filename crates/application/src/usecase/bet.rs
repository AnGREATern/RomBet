use anyhow::Result;

use domain::entity::{Bet, Game};
use domain::value_object::{Amount, BetStatistics, Coefficient, Event};

pub trait MakeBet {
    fn make_bet(
        &mut self,
        game: &Game,
        amount: Amount,
        event: Event,
        coefficient: Coefficient,
    ) -> Result<()>;

    fn calculate_coefficients(&mut self, game: &Game) -> Result<Vec<(Event, Coefficient)>>;

    fn calculate_winner_coefficients(&mut self, game: &Game) -> Result<Vec<(Event, Coefficient)>>;

    fn calculate_total_coefficients(&mut self, game: &Game) -> Result<Vec<(Event, Coefficient)>>;
}

pub trait CalculateBet {
    fn calculate_bets(&mut self) -> Result<f64>;

    fn calculate_bet(&mut self, bet: Bet) -> Result<f64>;
}

pub trait MakeReport {
    fn make_report(&mut self, start_balance: Amount) -> BetStatistics;
}

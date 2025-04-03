use super::Game;
use crate::value_object::{Amount, Coefficient, Event, Id};

pub struct Bet {
    id: Id<Bet>,
    amount: Amount,
    coefficient: Coefficient,
    game_id: Id<Game>,
    event: Event,
    is_won: Option<bool>,
}

impl Bet {
    pub fn new(
        id: Id<Bet>,
        amount: Amount,
        coefficient: Coefficient,
        game_id: Id<Game>,
        event: Event,
    ) -> Self {
        let is_won = None;
        Self {
            id,
            amount,
            coefficient,
            game_id,
            event,
            is_won,
        }
    }

    pub fn game_id(&self) -> Id<Game> {
        self.game_id
    }

    pub fn event(&self) -> Event {
        self.event
    }

    pub fn set_win(&mut self) -> f64 {
        self.is_won = Some(true);

        f64::from(self.amount) * f64::from(self.coefficient)
    }

    pub fn set_lose(&mut self) -> f64 {
        self.is_won = Some(false);

        self.amount.into()
    }
}

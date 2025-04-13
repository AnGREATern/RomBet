use super::{Game, Simulation};
use crate::value_object::{Amount, Coefficient, Event, Id};

pub struct Bet {
    id: Id<Bet>,
    simulation_id: Id<Simulation>,
    amount: Amount,
    coefficient: Coefficient,
    game_id: Id<Game>,
    event: Event,
    is_won: Option<bool>,
}

impl Bet {
    pub fn new(
        id: Id<Self>,
        simulation_id: Id<Simulation>,
        amount: Amount,
        coefficient: Coefficient,
        game_id: Id<Game>,
        event: Event,
        is_won: Option<bool>
    ) -> Self {
        Self {
            id,
            simulation_id,
            amount,
            coefficient,
            game_id,
            event,
            is_won,
        }
    }

    pub fn id(&self) -> Id<Self> {
        self.id
    }

    pub fn simulation_id(&self) -> Id<Simulation> {
        self.simulation_id
    }

    pub fn game_id(&self) -> Id<Game> {
        self.game_id
    }

    pub fn event(&self) -> Event {
        self.event
    }

    pub fn coefficient(&self) -> Coefficient {
        self.coefficient
    }

    pub fn amount(&self) -> Amount {
        self.amount
    }

    pub fn is_won(&self) -> Option<bool> {
        self.is_won
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

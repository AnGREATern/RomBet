use crate::value_object::{Id, Amount, Coefficient, Event};
use super::Game;

pub struct Bet {
    id: Id<Bet>,
    amount: Amount,
    coefficient: Coefficient,
    game_id: Id<Game>,
    event: Event,
}

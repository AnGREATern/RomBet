use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub enum Event {
    WDL(Winner),
    T(EventTotal),
}

#[derive(Clone, Copy)]
pub struct EventTotal {
    pub total: u8,
    pub ordering: Ordering,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    W1,
    X,
    W2,
}

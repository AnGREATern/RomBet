use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    WDL(Winner),
    T(EventTotal),
}

#[derive(Clone, Copy, Debug)]
pub struct EventTotal {
    pub total: u8,
    pub ordering: Ordering,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Winner {
    W1,
    X,
    W2,
}

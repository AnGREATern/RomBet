mod restart;

pub use restart::Restart;

pub trait ISessionService: Restart {}
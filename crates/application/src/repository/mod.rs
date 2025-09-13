mod bet;
mod game;
mod game_stat;
mod simulation;
mod team;

pub use bet::IBetRepo;
pub use game::IGameRepo;
pub use game_stat::IGameStatRepo;
pub use simulation::ISimulationRepo;
pub use team::ITeamRepo;

#[cfg(test)]
pub use bet::MockIBetRepo;
#[cfg(test)]
pub use game::MockIGameRepo;
#[cfg(test)]
pub use game_stat::MockIGameStatRepo;
#[cfg(test)]
pub use simulation::MockISimulationRepo;
#[cfg(test)]
pub use team::MockITeamRepo;

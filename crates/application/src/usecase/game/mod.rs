mod randomize_round;
mod create_round;

pub use randomize_round::RandomizeRound;
pub use create_round::CreateRound;

pub trait IGameService: RandomizeRound + CreateRound {}
use anyhow::Result;

use domain::{
    entity::{Game, GameStat},
    value_object::{Id, Winner},
};

pub trait IGameStatRepo {
    fn new() -> Self;

    fn add(&self, game_stat: GameStat) -> Result<Id<GameStat>>;

    fn game_stat_by_id(&self, game_stat_id: Id<GameStat>) -> Result<GameStat>;

    fn game_stat_by_game_id(&self, game_id: Id<Game>) -> Result<GameStat>;

    fn winner_by_game_id(&self, game_id: Id<Game>, is_home: bool) -> Result<Winner>;

    fn score_by_game_id(&self, game_id: Id<Game>, is_home: bool) -> Result<(u8, u8)>;

    fn goals_by_game_id(&self, game_id: Id<Game>, is_home: bool) -> Result<u8>;

    fn next_id(&self) -> Id<GameStat>;

    fn reset(&self);
}

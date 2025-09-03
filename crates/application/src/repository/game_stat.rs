use anyhow::Result;

use domain::{
    entity::{Game, GameStat},
    value_object::{Id, Winner},
};

pub trait IGameStatRepo {
    fn add(&self, game_stat: GameStat) -> Result<()>;

    fn game_stat_by_game_id(&self, game_id: Id<Game>) -> Result<GameStat>;

    fn winner_by_game_id(&self, game_id: Id<Game>, is_home: bool) -> Option<Winner>;

    fn score_by_game_id(&self, game_id: Id<Game>, is_home: bool) -> Option<(u8, u8)>;

    fn goals_by_game_id(&self, game_id: Id<Game>, is_home: bool) -> Option<u8>;

    fn next_id(&self) -> Id<GameStat>;
}

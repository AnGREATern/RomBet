use anyhow::Result;

use domain::{
    entity::{Game, GameStat},
    value_object::{Id, Winner},
};

pub trait IGameStatRepo {
    fn new() -> Self;

    fn add(&self, game_stat: GameStat) -> Result<Id<GameStat>>;

    fn game_stat_by_id(&self, game_stat_id: Id<GameStat>) -> Result<GameStat>;

    fn winner_by_game_id(&self, game_id: Id<Game>) -> Result<Winner>;

    fn num_of_games_with_total_x_more(&self, game_id: Id<Game>, x: f32);

    fn num_of_games_with_total_x_less(&self, game_id: Id<Game>, x: f32);

    fn next_id(&self) -> Id<GameStat>;

    fn reset(&self);
}
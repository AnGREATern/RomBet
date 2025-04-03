use super::Game;
use crate::value_object::Id;

pub struct GameStat {
    id: Id<GameStat>,
    game_id: Id<Game>,
    home_team_total: u8,
    guest_team_total: u8,
}

impl GameStat {
    pub fn new(
        id: Id<GameStat>,
        game_id: Id<Game>,
        home_team_total: u8,
        guest_team_total: u8,
    ) -> Self {
        Self {
            id,
            game_id,
            home_team_total,
            guest_team_total,
        }
    }
}

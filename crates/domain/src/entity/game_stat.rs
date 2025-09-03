use super::Game;
use crate::value_object::Id;

#[derive(Clone, Copy)]
pub struct GameStat {
    id: Id<Self>,
    game_id: Id<Game>,
    home_team_total: u8,
    guest_team_total: u8,
}

impl GameStat {
    pub fn new(id: Id<Self>, game_id: Id<Game>, home_team_total: u8, guest_team_total: u8) -> Self {
        Self {
            id,
            game_id,
            home_team_total,
            guest_team_total,
        }
    }

    pub fn id(&self) -> Id<Self> {
        self.id
    }

    pub fn game_id(&self) -> Id<Game> {
        self.game_id
    }

    pub fn home_team_total(&self) -> u8 {
        self.home_team_total
    }

    pub fn guest_team_total(&self) -> u8 {
        self.guest_team_total
    }
}

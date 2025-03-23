use crate::value_object::Id;
use super::Team;

pub struct Game {
    id: Id<Game>,
    home_team_id: Id<Team>,
    guest_team_id: Id<Team>,
    round: u32,
}

impl Game {
    pub fn new(id: Id<Game>, home_team_id: Id<Team>, guest_team_id: Id<Team>, round: u32) -> Self {
        Self {
            id,
            home_team_id,
            guest_team_id,
            round,
        }
    }

    pub fn home_team_id(&self) -> Id<Team> {
        self.home_team_id
    }

    pub fn guest_team_id(&self) -> Id<Team> {
        self.guest_team_id
    }
}
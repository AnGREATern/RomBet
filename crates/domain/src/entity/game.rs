use super::{Simulation, Team};
use crate::value_object::Id;

pub struct Game {
    id: Id<Game>,
    simulation_id: Id<Simulation>,
    home_team_id: Id<Team>,
    guest_team_id: Id<Team>,
    round: u32,
}

impl Game {
    pub fn new(
        id: Id<Game>,
        simulation_id: Id<Simulation>,
        home_team_id: Id<Team>,
        guest_team_id: Id<Team>,
        round: u32,
    ) -> Self {
        Self {
            id,
            simulation_id,
            home_team_id,
            guest_team_id,
            round,
        }
    }

    pub fn id(&self) -> Id<Self> {
        self.id
    }

    pub fn simulation_id(&self) -> Id<Simulation> {
        self.simulation_id
    }

    pub fn home_team_id(&self) -> Id<Team> {
        self.home_team_id
    }

    pub fn guest_team_id(&self) -> Id<Team> {
        self.guest_team_id
    }
}

use anyhow::{Result, bail};
use rand::rng;
use rand::seq::SliceRandom;
use std::net::IpAddr;

use crate::{
    config::SetupConfig,
    repository::{IGameRepo, IGameStatRepo, ISimulationRepo, ITeamRepo},
    usecase::{CreateRound, Start},
};
use domain::{
    entity::{Game, Simulation},
    value_object::Id,
};

const TEAMS_PER_GAME: usize = 2;

pub struct SimulationService<G: IGameRepo, T: ITeamRepo, GS: IGameStatRepo, S: ISimulationRepo> {
    game_repo: G,
    team_repo: T,
    game_stat_repo: GS,
    simulation_repo: S,
    config: SetupConfig,
}

impl<G: IGameRepo, T: ITeamRepo, GS: IGameStatRepo, S: ISimulationRepo> Start
    for SimulationService<G, T, GS, S>
{
    fn start(&mut self, ip: IpAddr) -> Result<Id<Simulation>> {
        if let Some(id) = self.simulation_repo.simulation_by_ip(ip) {
            Ok(id)
        } else {
            let id = self.simulation_repo.next_id();
            self.simulation_repo.add(Simulation::new(id, ip, self.config.balance))?;

            Ok(id)
        }
    }

    fn restart(&mut self, simulation: Simulation) -> Result<Id<Simulation>> {
        let ip = simulation.ip();
        self.simulation_repo.remove_by_id(simulation.id());

        self.start(ip)
    }
}

impl<G: IGameRepo, T: ITeamRepo, GS: IGameStatRepo, S: ISimulationRepo> CreateRound
    for SimulationService<G, T, GS, S>
{
    fn create_round(&mut self, simulation: &mut Simulation) -> Result<()> {
        let round = simulation.round();
        let simulation_id = simulation.id();
        self.check_last_round_randomized(round, simulation_id)?;
        let mut teams = self.team_repo.all_teams_id();
        teams.shuffle(&mut rng());
        for h2h in teams.chunks_exact(TEAMS_PER_GAME) {
            let home_team_id = h2h[0];
            let guest_team_id = h2h[1];
            let game_id = self.game_repo.next_id();
            let game = Game::new(
                game_id,
                simulation_id,
                home_team_id,
                guest_team_id,
                round + 1,
            );
            self.game_repo.add(game)?;
        }
        simulation.increment_round();

        Ok(())
    }
}

impl<G: IGameRepo, T: ITeamRepo, GS: IGameStatRepo, S: ISimulationRepo>
    SimulationService<G, T, GS, S>
{
    pub fn new(
        game_repo: G,
        team_repo: T,
        game_stat_repo: GS,
        simulation_repo: S,
        config: SetupConfig,
    ) -> Self {
        Self {
            game_repo,
            team_repo,
            game_stat_repo,
            simulation_repo,
            config,
        }
    }

    fn check_last_round_randomized(&mut self, round: u32, simulation_id: Id<Simulation>) -> Result<()> {
        let games_id = self.game_repo.games_id_by_round(round, simulation_id)?;
        for game_id in games_id {
            if self
                .game_stat_repo
                .game_stat_by_game_id(game_id)
                .ok()
                .is_none()
            {
                bail!("Last round didn't random");
            }
        }

        Ok(())
    }
}

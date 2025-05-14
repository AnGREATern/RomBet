use anyhow::{Result, bail};
use rand::rng;
use rand::seq::SliceRandom;
use std::net::IpAddr;
use std::fmt;

use crate::{
    config::SetupConfig,
    repository::{IGameRepo, IGameStatRepo, ISimulationRepo, ITeamRepo},
    usecase::{CreateRound, Start},
};
use domain::{
    entity::{Game, Team, Simulation},
    value_object::Id,
};

const TEAMS_PER_GAME: usize = 2;

pub struct DisplayedGame {
    pub id: Id<Game>,
    pub home_team: Team,
    pub guest_team: Team,
}

impl DisplayedGame {
    fn new(game: Game, team_repo: &mut impl ITeamRepo) -> Result<Self> {
        let id = game.id();
        let home_team = team_repo.team_by_id(game.home_team_id())?;
        let guest_team = team_repo.team_by_id(game.guest_team_id())?;

        Ok(Self { id, home_team, guest_team })
    }
}

impl fmt::Display for DisplayedGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.home_team.name(), self.guest_team.name())
    }
}

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
    fn start(&mut self, ip: IpAddr) -> Result<Simulation> {
        if let Some(simulation) = self.simulation_repo.simulation_by_ip(ip) {
            Ok(simulation)
        } else {
            let id = self.simulation_repo.next_id();
            let simulation = Simulation::new(id, ip, self.config.balance);
            self.simulation_repo.add(simulation)?;

            Ok(simulation)
        }
    }

    fn restart(&mut self, simulation_id: Id<Simulation>) -> Result<Simulation> {
        let ip = self.simulation_repo.simulation_by_id(simulation_id)?.ip();
        self.simulation_repo.remove_by_id(simulation_id);

        self.start(ip)
    }
}

impl<G: IGameRepo, T: ITeamRepo, GS: IGameStatRepo, S: ISimulationRepo> CreateRound
    for SimulationService<G, T, GS, S>
{
    fn create_round(&mut self, simulation: &mut Simulation) -> Result<Vec<DisplayedGame>> {
        let mut round = simulation.round();
        let simulation_id = simulation.id();
        self.check_last_round_randomized(round, simulation_id)?;
        round += 1;
        simulation.increment_round();
        self.simulation_repo.update_by_id(simulation.clone())?;
        let mut teams = self.team_repo.all_teams_id();
        teams.shuffle(&mut rng());
        let mut displayed_games = vec![];
        for h2h in teams.chunks_exact(TEAMS_PER_GAME) {
            let home_team_id = h2h[0];
            let guest_team_id = h2h[1];
            let game_id = self.game_repo.next_id();
            let game = Game::new(
                game_id,
                simulation_id,
                home_team_id,
                guest_team_id,
                round,
            );
            self.game_repo.add(game)?;
            let displayed_game = DisplayedGame::new(game, &mut self.team_repo)?;
            displayed_games.push(displayed_game);
        }

        Ok(displayed_games)
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

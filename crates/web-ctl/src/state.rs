use std::net::IpAddr;

use anyhow::Result;
use application::usecase::Start;
use domain::entity::Simulation;
use tracing::debug;

use application::config::{AppConfig, SetupConfig};
use application::service::{BetService, GameService, SimulationService, TeamService};
use db::init_pool;
use db::repository::{BetRepo, GameRepo, GameStatRepo, SimulationRepo, TeamRepo};

pub struct AppState {
    sim_service: SimulationService<GameRepo, TeamRepo, GameStatRepo, SimulationRepo>,
    game_service: GameService<GameRepo, GameStatRepo, TeamRepo>,
    bet_service: BetService<BetRepo, GameRepo, GameStatRepo, SimulationRepo>,
    team_service: TeamService<TeamRepo>,
    team_repo: TeamRepo,
    game_repo: GameRepo,
    setup_config: SetupConfig,
}

impl TryFrom<AppConfig> for AppState {
    type Error = anyhow::Error;

    fn try_from(config: AppConfig) -> Result<Self, Self::Error> {
        let setup_config = config.setup;
        let coefficient_config = config.coefficient;

        let pool = init_pool();
        let game_repo = GameRepo::new(pool.clone());
        let bet_repo = BetRepo::new(pool.clone());
        let game_stat_repo = GameStatRepo::new(pool.clone());
        let simulation_repo = SimulationRepo::new(pool.clone());
        let bet_service = BetService::new(
            bet_repo,
            game_repo,
            game_stat_repo,
            simulation_repo,
            coefficient_config.clone(),
        );
        debug!("Bet service started");

        let game_repo = GameRepo::new(pool.clone());
        let game_stat_repo = GameStatRepo::new(pool.clone());
        let team_repo = TeamRepo::new(pool.clone());
        let game_service =
            GameService::new(game_repo, game_stat_repo, team_repo, coefficient_config);
        debug!("Game service started");

        let team_repo = TeamRepo::new(pool.clone());
        let game_repo = GameRepo::new(pool.clone());
        let simulation_repo = SimulationRepo::new(pool.clone());
        let game_stat_repo = GameStatRepo::new(pool.clone());
        let sim_service = SimulationService::new(
            game_repo,
            team_repo,
            game_stat_repo,
            simulation_repo,
            setup_config,
        );
        debug!("Simulation service started");

        let team_repo = TeamRepo::new(pool.clone());
        let team_service = TeamService::new(team_repo);
        debug!("Team service started");

        let team_repo = TeamRepo::new(pool.clone());

        let game_repo = GameRepo::new(pool);

        Ok(Self {
            game_service,
            bet_service,
            sim_service,
            team_service,
            team_repo,
            game_repo,
            setup_config,
        })
    }
}

impl AppState {
    pub fn simulation_service(
        &self,
    ) -> &SimulationService<GameRepo, TeamRepo, GameStatRepo, SimulationRepo> {
        &self.sim_service
    }

    pub fn game_service(&self) -> &GameService<GameRepo, GameStatRepo, TeamRepo> {
        &self.game_service
    }

    pub fn bet_service(&self) -> &BetService<BetRepo, GameRepo, GameStatRepo, SimulationRepo> {
        &self.bet_service
    }

    pub fn team_repo(&self) -> &TeamRepo {
        &self.team_repo
    }

    pub fn game_repo(&self) -> &GameRepo {
        &self.game_repo
    }

    pub fn team_service(&self) -> &TeamService<TeamRepo> {
        &self.team_service
    }

    pub fn simulation(&self, ip: IpAddr) -> Result<Simulation> {
        self.sim_service.start(ip)
    }

    pub fn setup_config(&self) -> &SetupConfig {
        &self.setup_config
    }
}

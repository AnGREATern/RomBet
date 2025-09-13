use anyhow::anyhow;
use anyhow::{Result, bail};
use clap::Parser;
use dotenv::dotenv;
use enum_try_from::impl_enum_try_from;
use std::collections::BTreeMap;
use std::fmt;
use std::io;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;
use std::process::ExitCode;
use tracing::warn;
use tracing::{debug, error, info};

use application::config::SetupConfig;
use application::service::{BetService, GameService, SimulationService};
use application::usecase::{CalculateBet, CreateRound, MakeBet, MakeReport, RandomizeRound, Start};
use db::init_pool;
use db::repository::{BetRepo, GameRepo, GameStatRepo, SimulationRepo, TeamRepo};
use domain::entity::{Game, Simulation, Team};
use domain::value_object::{Amount, Coefficient, Event, Id, MIN_BALANCE_AMOUNT, MIN_BET_AMOUNT};
use infrastructure::{config, logger};

#[derive(Parser)]
#[command(version, about = "The best betting emulator!", long_about = None)]
struct CliArgs {
    #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
    pub ip: IpAddr,
}

impl_enum_try_from!(
    #[repr(u8)]
    #[derive(Default, PartialEq, Eq)]
    pub enum Command {
        #[default]
        Start = 0,
        Restart,
        CreateRound,
        RandomizeRound,
        CalculateCoefficients,
        MakeReport,
        CheckBalance,
        Exit,
    },
    u8,
    (),
    ()
);

struct GameInfo {
    pub home_team: Team,
    pub guest_team: Team,
    pub home_team_score: Option<u8>,
    pub guest_team_score: Option<u8>,
}

impl fmt::Display for GameInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.home_team_score.is_some() && self.guest_team_score.is_some() {
            write!(
                f,
                "{} {} - {} {}",
                self.home_team.name(),
                self.home_team_score.unwrap(),
                self.guest_team_score.unwrap(),
                self.guest_team.name()
            )
        } else {
            write!(
                f,
                "{} ? - ? {}",
                self.home_team.name(),
                self.guest_team.name()
            )
        }
    }
}

struct App {
    sim_service: SimulationService<GameRepo, TeamRepo, GameStatRepo, SimulationRepo>,
    game_service: GameService<GameRepo, GameStatRepo, TeamRepo>,
    bet_service: BetService<BetRepo, GameRepo, GameStatRepo, SimulationRepo>,
    simulation: Simulation,
    games: BTreeMap<Id<Game>, GameInfo>,
    game_poses: Vec<Id<Game>>,
    setup_config: SetupConfig,
}

impl App {
    pub fn new(cli_args: CliArgs) -> Result<Self> {
        dotenv().ok();
        logger::init_default_logger();
        let config = config::load_from_file(Path::new("config.toml"))?;
        info!("Config applied");
        let setup_config = config.setup;
        let coefficient_config = config.coefficient;

        let games = BTreeMap::new();
        let game_poses = vec![];

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

        let simulation = sim_service.start(cli_args.ip)?;
        debug!("Current round: {}", simulation.round());
        info!(
            balance = f64::from(simulation.balance()),
            "Simulation started successfully"
        );

        Ok(Self {
            simulation,
            game_service,
            bet_service,
            sim_service,
            game_poses,
            games,
            setup_config,
        })
    }

    pub fn show_menu() {
        debug!("Menu showed");
        println!("-----Меню-----");
        println!("{}. Начать сначала", Command::Restart as u8);
        println!("{}. Перейти к следующему туру", Command::CreateRound as u8);
        println!(
            "{}. Сгенерировать результаты тура",
            Command::RandomizeRound as u8
        );
        println!(
            "{}. Получить коэффициенты на матч",
            Command::CalculateCoefficients as u8
        );
        println!("{}. Статистика ставок", Command::MakeReport as u8);
        println!("{}. Посмотреть баланс", Command::CheckBalance as u8);
        println!("{}. Выход", Command::Exit as u8);
        println!("--------------");
    }

    pub fn perform(&mut self, cmd: &Command) -> Result<()> {
        match cmd {
            Command::Restart => self.restart(),
            Command::RandomizeRound => self.randomize_round(),
            Command::CalculateCoefficients => self.calculate_coefficients(),
            Command::CreateRound => self.create_round(),
            Command::MakeReport => self.make_report(),
            Command::CheckBalance => self.check_balance(),
            Command::Exit => Ok(()),
            _ => bail!("Undefined command"),
        }
    }

    fn restart(&mut self) -> Result<()> {
        debug!("Perform restart operation");
        self.simulation = self.sim_service.restart(self.simulation.id())?;
        info!(
            balance = f64::from(self.simulation.balance()),
            "Restart successful"
        );

        Ok(())
    }

    fn randomize_round(&mut self) -> Result<()> {
        debug!("Perform randomize round operation");
        let games_stat = self.game_service.randomize_round(&self.simulation)?;
        if games_stat.is_empty() {
            info!("All the matches have been played");
            return Ok(());
        }

        info!(round = self.simulation.round(), "Show game results");
        println!("Результаты матчей {}-го тура:", self.simulation.round());
        for game_stat in games_stat {
            println!("{}", game_stat);
        }
        let profit = self.bet_service.calculate_bets()?;
        info!(profit = f64::from(profit), "Credit to balance");
        println!(
            "Доход по итогам ставок на матчи этого тура: {}",
            f64::from(profit)
        );
        self.simulation.process_bet(profit)?;
        self.create_round()?;

        Ok(())
    }

    fn calculate_coefficients(&mut self) -> Result<()> {
        debug!("Perform calculate coefficients operation");
        if self.game_poses.is_empty() {
            info!("Round didn't create");
            println!("Сначала посмотрите матчи тура!");
            return Ok(());
        }
        let mut buffer = String::new();
        println!("Введите номер матча: ");
        io::stdin().read_line(&mut buffer)?;
        let game_pos = buffer.trim().parse::<usize>()?;
        let game_id = self.game_poses.get(game_pos).ok_or_else(|| {
            println!("Такого матча нет");
            warn!("Incorrect game pos");
            anyhow!("Incorrect game pos")
        })?;
        let game_info = self.games.get(&game_id).unwrap();
        let game = Game::new(
            *game_id,
            self.simulation.id(),
            game_info.home_team.id(),
            game_info.guest_team.id(),
            self.simulation.round(),
        );
        info!("Game selected");
        let offers = self.bet_service.calculate_coefficients(&game)?;
        for (i, (event, coefficient)) in offers.iter().enumerate() {
            println!("{}. {} за {}", i, event, f64::from(*coefficient));
        }
        info!("Coefficients calculated");
        println!(
            "Введите номер события, на которое хотите сделать ставку \
            или любое другое число, чтобы вернуться в меню:"
        );
        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        let event_pos = buffer.trim().parse::<usize>()?;
        if event_pos < offers.len() {
            debug!("Game selected");
            let (event, coefficient) = offers[event_pos];
            self.make_bet(&game, event, coefficient)
        } else {
            debug!("Game didn't select");
            Ok(())
        }
    }

    fn create_round(&mut self) -> Result<()> {
        debug!("Perform create round operation");
        let _ = self.randomize_round();
        self.game_poses.clear();
        let games = self.sim_service.create_round(&mut self.simulation)?;

        info!(round = self.simulation.round(), "Games created");
        println!("Матчи {}-го тура:", self.simulation.round());
        for (i, game) in games.into_iter().enumerate() {
            println!("{}. {}", i, game);
            self.games.insert(
                game.id,
                GameInfo {
                    home_team: game.home_team,
                    guest_team: game.guest_team,
                    home_team_score: None,
                    guest_team_score: None,
                },
            );
            self.game_poses.push(game.id);
        }

        Ok(())
    }

    fn make_report(&mut self) -> Result<()> {
        debug!("Perform make report operation");
        let stat = self.bet_service.make_report(self.setup_config.balance);
        print!(
            "Ваша статистика:\nНачальный баланс: {}\nМинимальный проигравший коэффициент: ",
            f64::from(stat.start_balance())
        );
        if let Some(c) = stat.min_coefficient_lose() {
            println!("{}", f64::from(c));
        } else {
            println!("-");
        }

        Ok(())
    }

    fn make_bet(&mut self, game: &Game, event: Event, coefficient: Coefficient) -> Result<()> {
        debug!("Perform make bet operation");
        self.check_balance()?;
        println!("Введите сумму ставки: ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let value = buffer.trim().parse::<f64>()?;
        let value = Amount::new_with_casting(value, Some(MIN_BALANCE_AMOUNT))?;
        debug!("Sum of bet parsed");
        if MIN_BET_AMOUNT <= value.clear_value()
            && value.clear_value() <= self.simulation.balance().clear_value()
        {
            info!("All conditions to make bet passed");
            self.simulation.make_bet(value)?;
            info!("Bet made");
            self.bet_service.make_bet(game, value, event, coefficient)
        } else if MIN_BET_AMOUNT <= value.clear_value() {
            bail!("Haven't enough money");
        } else {
            bail!(
                "Minimal bet is {}",
                f64::from(Amount::new(MIN_BET_AMOUNT, None).unwrap())
            );
        }
    }

    fn check_balance(&self) -> Result<()> {
        debug!("Perform check balance operation");
        println!("Ваш баланс: {}", f64::from(self.simulation.balance()));

        Ok(())
    }
}

fn main() -> ExitCode {
    let cli = CliArgs::parse();
    let mut buffer = String::new();
    let mut cmd = Command::default();
    let mut app = match App::new(cli) {
        Ok(app) => app,
        Err(error) => {
            error!("{}", error);
            return ExitCode::FAILURE;
        }
    };
    info!("App started");

    while cmd != Command::Exit {
        App::show_menu();

        buffer.clear();
        if let Err(_) = io::stdin().read_line(&mut buffer) {
            println!("Строка не считана. Попробуйте ещё раз");
            continue;
        }

        let dig_buf = buffer.trim().parse::<u8>();
        if let Err(_) = dig_buf {
            println!("Число не распознано. Попробуйте ещё раз");
            continue;
        }

        let cmd_buf = Command::try_from(dig_buf.unwrap());
        if let Err(_) = cmd_buf {
            println!("Команда не распознана. Попробуйте ещё раз");
            continue;
        }

        cmd = cmd_buf.unwrap();
        info!("Command parsed");
        if let Err(error) = app.perform(&cmd) {
            error!("{}", error);
        }
    }
    info!("Shutdown application");

    ExitCode::SUCCESS
}

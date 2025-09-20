mod bet;
mod game;
mod game_stat;
mod simulation;
mod team;

pub use bet::BetRepo;
pub use game::GameRepo;
pub use game_stat::GameStatRepo;
pub use simulation::SimulationRepo;
pub use team::TeamRepo;

#[cfg(test)]
mod common {
    use std::net::{IpAddr, Ipv4Addr};

    use diesel::{RunQueryDsl, SqliteConnection};
    use domain::{
        entity::{Game, Simulation},
        value_object::{Amount, Id, MIN_BALANCE_AMOUNT},
    };
    use uuid::Uuid;

    pub fn run_migrations(connection: &mut SqliteConnection) {
        diesel::sql_query(include_str!(
            "../../migrations/2025-04-08-152635_create_team/up.sql"
        ))
        .execute(connection)
        .unwrap();
        diesel::sql_query(
            "INSERT INTO Team (id, name) VALUES
            ('123e4567-e89b-12d3-a456-426614174000', 'Спартак'),
            ('123e4567-e89b-12d3-a456-426614174001', 'Зенит'),
            ('123e4567-e89b-12d3-a456-426614174002', 'Динамо'),
            ('123e4567-e89b-12d3-a456-426614174003', 'Локомотив'),
            ('123e4567-e89b-12d3-a456-426614174004', 'Краснодар'),
            ('123e4567-e89b-12d3-a456-426614174005', 'Ростов'),
            ('123e4567-e89b-12d3-a456-426614174006', 'Сочи'),
            ('123e4567-e89b-12d3-a456-426614174007', 'Урал'),
            ('123e4567-e89b-12d3-a456-426614174008', 'Ахмат'),
            ('123e4567-e89b-12d3-a456-426614174009', 'Рубин'),
            ('123e4567-e89b-12d3-a456-426614174010', 'Крылья Советов'),
            ('123e4567-e89b-12d3-a456-426614174011', 'Тамбов'),
            ('123e4567-e89b-12d3-a456-426614174012', 'Уфа'),
            ('123e4567-e89b-12d3-a456-426614174013', 'Химки'),
            ('123e4567-e89b-12d3-a456-426614174014', 'Ротор')",
        )
        .execute(connection)
        .unwrap();
        diesel::sql_query(include_str!(
            "../../migrations/2025-04-08-155152_create_simulation/up.sql"
        ))
        .execute(connection)
        .unwrap();
        diesel::sql_query(include_str!(
            "../../migrations/2025-04-08-155525_create_game/up.sql"
        ))
        .execute(connection)
        .unwrap();
        diesel::sql_query(include_str!(
            "../../migrations/2025-04-08-155926_create_bet/up.sql"
        ))
        .execute(connection)
        .unwrap();
        diesel::sql_query(include_str!(
            "../../migrations/2025-04-08-160321_create_game_stat/up.sql"
        ))
        .execute(connection)
        .unwrap();
    }

    pub struct SimulationBuilder {
        id: Id<Simulation>,
        ip: IpAddr,
        round: Option<u32>,
        balance: Amount,
    }

    impl SimulationBuilder {
        pub fn new() -> Self {
            Self {
                id: Id::new(),
                ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
                round: None,
                balance: Amount::new(1000_00, Some(MIN_BALANCE_AMOUNT)).unwrap(),
            }
        }

        pub fn id(mut self, id: Id<Simulation>) -> Self {
            self.id = id;
            self
        }

        pub fn ip(mut self, ip: IpAddr) -> Self {
            self.ip = ip;
            self
        }

        pub fn round(mut self, round: u32) -> Self {
            self.round = Some(round);
            self
        }

        pub fn balance(mut self, balance: Amount) -> Self {
            self.balance = balance;
            self
        }

        pub fn build(self) -> Simulation {
            Simulation::new(self.id, self.ip, self.balance, self.round)
        }
    }

    pub struct GameFactory;

    impl GameFactory {
        pub fn create_spa_zen_game(simulation_id: Id<Simulation>) -> Game {
            Game::new(
                Id::new(),
                simulation_id,
                Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000")
                    .unwrap()
                    .into(),
                Uuid::parse_str("123e4567-e89b-12d3-a456-426614174001")
                    .unwrap()
                    .into(),
                1,
            )
        }

        pub fn create_din_lok_game(simulation_id: Id<Simulation>) -> Game {
            Game::new(
                Id::new(),
                simulation_id,
                Uuid::parse_str("123e4567-e89b-12d3-a456-426614174002")
                    .unwrap()
                    .into(),
                Uuid::parse_str("123e4567-e89b-12d3-a456-426614174003")
                    .unwrap()
                    .into(),
                1,
            )
        }

        pub fn create_kra_ros_game(simulation_id: Id<Simulation>) -> Game {
            Game::new(
                Id::new(),
                simulation_id,
                Uuid::parse_str("123e4567-e89b-12d3-a456-426614174004")
                    .unwrap()
                    .into(),
                Uuid::parse_str("123e4567-e89b-12d3-a456-426614174005")
                    .unwrap()
                    .into(),
                1,
            )
        }
    }
}

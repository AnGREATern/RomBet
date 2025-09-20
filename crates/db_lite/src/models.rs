use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::Team)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeamSqlite {
    pub id: String,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = crate::schema::Bet)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BetSqlite {
    pub id: String,
    pub simulation_id: String,
    pub amount: i64,
    pub coefficient: i32,
    pub game_id: String,
    pub event: Vec<u8>,
    pub is_won: Option<bool>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::GameStat)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GameStatSqlite {
    pub id: String,
    pub game_id: String,
    pub home_team_total: i16,
    pub guest_team_total: i16,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::Game)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GameSqlite {
    pub id: String,
    pub simulation_id: String,
    pub home_team_id: String,
    pub guest_team_id: String,
    pub round: i64,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::Simulation)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SimulationSqlite {
    pub id: String,
    pub ip: String,
    pub round: i64,
    pub balance: i64,
}

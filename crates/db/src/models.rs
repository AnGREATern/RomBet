use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::team)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TeamPostrgres {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::bet)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BetPostrgres {
    pub id: Uuid,
    pub simulation_id: Uuid,
    pub amount: i64,
    pub coefficient: i32,
    pub game_id: Uuid,
    pub event: Vec<u8>,
    pub is_won: Option<bool>,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::gamestat)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GameStatPostrgres {
    pub id: Uuid,
    pub game_id: Uuid,
    pub home_team_total: i16,
    pub guest_team_total: i16,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::game)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GamePostrgres {
    pub id: Uuid,
    pub simulation_id: Uuid,
    pub home_team_id: Uuid,
    pub guest_team_id: Uuid,
    pub round: i64,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::simulation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SimulationPostrgres {
    pub id: Uuid,
    pub ip: String,
    pub round: i64,
    pub balance: i64,
}

pub mod models;
pub mod repository;
pub mod schema;

use diesel::pg::PgConnection;
// use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

// pub type DBPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> DBPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let database_url = "sqlite::memory:";

    // let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(16)
        .build(manager)
        .expect("Failed to create DB pool")
}

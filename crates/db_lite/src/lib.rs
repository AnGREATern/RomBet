pub mod models;
pub mod repository;
pub mod schema;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub type DBPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool() -> DBPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .max_size(16)
        .build(manager)
        .expect("Failed to create DB pool")
}

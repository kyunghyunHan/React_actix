extern crate dotenv;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use std::env;
use std::ops::Deref;
// pub const DATABASE_FILE: &'static str = env!("DATABASE_URL");
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> DbPool {
    dotenv().ok(); // Grabbing ENV vars

    // Pull DATABASE_URL env var
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
pub struct Conn(pub PooledConnection<ConnectionManager<MysqlConnection>>);
impl Deref for Conn {
    type Target = MysqlConnection;

    // #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

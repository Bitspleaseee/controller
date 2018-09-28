use diesel::prelude::*;
use dotenv::dotenv;
use log::*;
use std::env;
use std::fmt;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

pub mod categories;
pub mod schema;
pub mod users;

/// The only errors returned to outside the `db` module
pub enum Error {
    Connection,
    Database,
    NotFound,
}

pub type DbConn = MysqlConnection;
pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Connection => write!(f, "Connection error"),
            Error::Database => write!(f, "Database error"),
            Error::NotFound => write!(f, "Item not found"),
        }
    }
}

/// Establishes a connection to the db
pub fn establish_connection() -> Option<MysqlConnection> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set"));

    let connection = MysqlConnection::establish(&database_url);

    if connection.is_err() {
        error!("Failed to connect to db");
        return None;
    }

    connection.ok()
}

pub fn setup_connection_pool() -> DbPool {
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set"));
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let diesel_db_config = r2d2::Pool::builder();
    diesel_db_config.build(manager).unwrap()
}

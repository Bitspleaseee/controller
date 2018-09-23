use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::fmt;

pub mod categories;
pub mod models;
pub mod schema;
pub mod users;

/// The only errors returned to outside the `db` module
pub enum Error {
    Connection,
    Database,
    NotFound,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Connection => write!(f, "Connection error"),
            Error::Database => write!(f, "Database error"),
            Error::NotFound => write!(f, "Item not found"),
        }
    }
}

/// Log string
fn log(text: &str) {
    println!("{}", text);
}

/// Establishes a connection to the db
pub fn establish_connection() -> Option<MysqlConnection> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set"));

    let connection = MysqlConnection::establish(&database_url);

    if connection.is_err() {
        log("Failed to connect to db");
        return None;
    }

    connection.ok()
}

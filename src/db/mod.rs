use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use failure::ResultExt;

use crate::{IntErrorKind, IntResult};

pub mod categories;
pub mod comments;
pub mod schema;
pub mod threads;
pub mod users;

pub type DbConn = MysqlConnection;
pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;

/// Establishes a connection to the database
pub fn establish_connection(database_url: &str) -> IntResult<DbConn> {
    MysqlConnection::establish(database_url)
        .context(IntErrorKind::ConnectionError)
        .map_err(|e| e.into())
}

/// Makes a pool of connections to the database
pub fn setup_connection_pool(database_url: &str) -> IntResult<DbPool> {
    let manager = ConnectionManager::<DbConn>::new(database_url);
    let diesel_db_config = r2d2::Pool::builder();

    diesel_db_config
        .build(manager)
        .context(IntErrorKind::ConnectionError)
        .map_err(|e| e.into())
}

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use failure::ResultExt;

use crate::{IntErrorKind, IntResult};

pub mod categories;
pub mod comments;
pub mod schema;
pub mod search;
pub mod threads;
pub mod users;

pub type DbConn = MysqlConnection;
pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;

const MAX_THREAD_LIMIT: i64 = 30;
const MAX_CATEGORY_LIMIT: i64 = 30;
const MAX_COMMENT_LIMIT: i64 = 30;
const MAX_SEARCH_LIMIT: i64 = 30;

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

#[cfg(test)]
mod _tests {
    use super::*;

    #[test]
    fn clear() {
        let con = establish_connection(&std::env::var("DATABASE_URL").unwrap()).unwrap();
        assert!(comments::delete_all_comments(&con).is_ok());
        assert!(threads::delete_all_threads(&con).is_ok());
        assert!(categories::delete_all_categories(&con).is_ok());
        assert!(users::delete_all_users(&con).is_ok());
    }
}

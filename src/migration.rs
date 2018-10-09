use diesel::prelude::*;
use diesel::sql_query;

use crate::db::establish_connection;
use crate::{IntErrorKind, IntResult};

pub fn run (database_url: &str) -> IntResult<()> {
    let con = establish_connection(database_url)?;

    sql_query(r#"CREATE TABLE users (

  id INT UNSIGNED NOT NULL,
  username VARCHAR(20) NOT NULL,
  description VARCHAR(255) NULL,
  avatar VARCHAR(36) NULL,

  PRIMARY KEY (id)
);"#).execute(&con).map_err(|_| IntErrorKind::QueryError)?;

    sql_query(r#"CREATE TABLE categories (

  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  title VARCHAR(45) NOT NULL,
  description TEXT NOT NULL,
  hidden BOOLEAN NOT NULL DEFAULT 0,

  PRIMARY KEY (id)
);"#).execute(&con).map_err(|_| IntErrorKind::QueryError)?;

    sql_query(r#"CREATE TABLE threads (

  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  category_id INT UNSIGNED NOT NULL,
  user_id INT UNSIGNED NOT NULL,
  title VARCHAR(45) NOT NULL,
  description TEXT NOT NULL,
  timestamp DATETIME NOT NULL DEFAULT NOW(),
  hidden BOOLEAN NOT NULL DEFAULT 0,

  PRIMARY KEY (id),

  FOREIGN KEY (category_id)
    REFERENCES categories(id),

  FOREIGN KEY (user_id)
    REFERENCES users(id)
);"#).execute(&con).map_err(|_| IntErrorKind::QueryError)?;

    sql_query(r#"CREATE TABLE comments (

  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  thread_id INT UNSIGNED NOT NULL,
  parent_id INT UNSIGNED NULL,
  user_id INT UNSIGNED NOT NULL,
  content TEXT NOT NULL,
  timestamp DATETIME NOT NULL DEFAULT NOW(),
  hidden BOOLEAN NOT NULL DEFAULT 0,

  PRIMARY KEY (id),

  FOREIGN KEY (thread_id)
    REFERENCES threads(id),

  FOREIGN KEY (parent_id)
    REFERENCES comments(id),

  FOREIGN KEY (user_id)
    REFERENCES users(id)
);"#).execute(&con).map_err(|_| IntErrorKind::QueryError)?;

    Ok(())
}
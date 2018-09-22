use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::User;

/// Establishes a connection to the db
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set"));
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Inserts new user into the user table
pub fn insert_user(user_id: i32, username: &str) -> Result<Option<User>, diesel::result::Error> {
    use self::schema::users::dsl::{id, users};

    let connection = establish_connection();

    let new_user = User {
        id: user_id,
        username: username.to_string(),
        description: None,
        avatar: None,
    };

    println!("Inserting user: ({}) {}", new_user.id, new_user.username);
    let result = diesel::insert_into(users)
        .values(&new_user)
        .execute(&connection);

    match result {
        Ok(_) => Ok(users.order(id.desc()).first(&connection).ok()),
        Err(error) => {
            println!(
                "Error inserting user: ({}) {}",
                new_user.id, new_user.username
            );
            Err(error)
        }
    }
}

/// Gets an exisiting user from the user table
pub fn get_user(user_id: i32) -> Result<Option<User>, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let connection = establish_connection();

    println!("Getting user: {}", user_id);
    users.filter(id.eq(user_id)).first(&connection).optional()
}

// Updates an existing user in the user table
pub fn update_user(user: &User) -> Result<usize, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let connection = establish_connection();

    let result = diesel::update(users)
        .set(user)
        .filter(id.eq(user.id))
        .execute(&connection);

    match result {
        Ok(num_updated) => {
            println!("Updated {} rows", num_updated);
            Ok(num_updated)
        }
        Err(error) => {
            println!("Error updating user: ({}) {}", user.id, user.username);
            Err(error)
        }
    }
}

// Deletes a existing user from the user table
pub fn delete_user(user_id: i32) -> Result<usize, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let connection = establish_connection();

    let result = diesel::delete(users)
        .filter(id.eq(user_id))
        .execute(&connection);

    match result {
        Ok(num_deleted) => {
            println!("Deleted {} rows", num_deleted);
            Ok(num_deleted)
        }
        Err(error) => {
            println!("Error deleting user: {}", user_id);
            Err(error)
        }
    }
}

// Clears the user table
pub fn delete_all_users() -> Result<usize, diesel::result::Error> {
    use self::schema::users::dsl::*;

    let connection = establish_connection();

    let result = diesel::delete(users).execute(&connection);

    match result {
        Ok(num_deleted) => {
            println!("Deleted {} rows", num_deleted);
            Ok(num_deleted)
        }
        Err(error) => {
            println!("Error deleting users");
            Err(error)
        }
    }
}

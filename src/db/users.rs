use diesel::prelude::*;

use super::{establish_connection, log, models::User, Error};

/// Inserts new user into the user table
pub fn insert_user(user_id: i32, user_name: &str) -> Result<User, Error> {
    use super::schema::users::dsl::{id, username, users};

    let connection = match establish_connection() {
        Some(con) => con,
        None => return Err(Error::Connection),
    };

    log(&format!("Inserting user ({}:{})", user_id, user_name));

    let result = diesel::insert_into(users)
        .values((id.eq(user_id), username.eq(user_name)))
        .execute(&connection);

    if result.is_err() {
        log(&format!(
            "Error inserting user ({}:{}): {}",
            user_id,
            user_name,
            result.err().unwrap()
        ));
        Err(Error::Database)
    } else {
        let result = users.filter(id.eq(user_id)).first::<User>(&connection);

        match result {
            Err(error) => {
                log(&format!(
                    "Error getting inserted user ({}): {}",
                    user_id, error
                ));
                Err(Error::Database)
            }
            Ok(user) => Ok(user),
        }
    }
}

/// Gets an exisiting user from the user table
fn get_user_con(connection: &diesel::MysqlConnection, user_id: i32) -> Result<User, Error> {
    use super::schema::users::dsl::{id, users};

    log(&format!("Getting user ({})", user_id));

    let result = users
        .filter(id.eq(user_id))
        .first::<User>(connection)
        .optional();

    match result {
        Err(error) => {
            log(&format!("Error getting user ({}): {}", user_id, error));
            Err(Error::Database)
        }
        Ok(row) => match row {
            None => Err(Error::NotFound),
            Some(user) => Ok(user),
        },
    }
}

/// Gets an exisiting user from the user table
pub fn get_user(user_id: i32) -> Result<User, Error> {
    let connection = match establish_connection() {
        Some(con) => con,
        None => return Err(Error::Connection),
    };

    get_user_con(&connection, user_id)
}

/// Deletes an existing user from the user table
pub fn delete_user(user_id: i32) -> Result<usize, Error> {
    use super::schema::users::dsl::{id, users};

    let connection = match establish_connection() {
        Some(con) => con,
        None => return Err(Error::Connection),
    };

    log(&format!("Deleting user ({})", user_id));

    let result = diesel::delete(users)
        .filter(id.eq(user_id))
        .execute(&connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted == 0 {
                Err(Error::NotFound)
            } else {
                Ok(num_deleted)
            }
        }
        Err(error) => {
            log(&format!("Error deleting user ({}): {}", user_id, error));
            Err(Error::Database)
        }
    }
}

/// Clears the user table
pub fn delete_all_users() -> Result<usize, Error> {
    use super::schema::users::dsl::users;

    let connection = match establish_connection() {
        Some(con) => con,
        None => return Err(Error::Connection),
    };

    log("Deleting all users");

    let result = diesel::delete(users).execute(&connection);

    match result {
        Ok(num_deleted) => Ok(num_deleted),
        Err(error) => {
            log(&format!("Error deleting all users: {}", error));
            Err(Error::Database)
        }
    }
}

/// Gets the updated user or an error based on the result of the update statement
fn get_update_result(
    result: Result<usize, diesel::result::Error>,
    connection: &diesel::MysqlConnection,
    user_id: i32,
) -> Result<User, Error> {
    match result {
        Ok(num_updated) => {
            if num_updated == 0 {
                Err(Error::NotFound)
            } else {
                get_user_con(&connection, user_id)
            }
        }
        Err(error) => {
            log(&format!("Error updating user ({}): {}", user_id, error));
            Err(Error::Database)
        }
    }
}

/// Updates an existing user in the user table
pub fn update_user(user: &User) -> Result<User, Error> {
    use super::schema::users::dsl::{id, users};

    let connection = match establish_connection() {
        Some(con) => con,
        None => return Err(Error::Connection),
    };

    log(&format!("Updating user ({}:{})", user.id, user.username));

    let result = diesel::update(users)
        .set(user)
        .filter(id.eq(user.id))
        .execute(&connection);

    get_update_result(result, &connection, user.id)
}

/// Updates the title for an existing user in the user table
pub fn update_user_username(user_id: i32, new_username: &str) -> Result<User, Error> {
    use super::schema::users::dsl::{id, username, users};

    let connection = match establish_connection() {
        Some(con) => con,
        None => return Err(Error::Connection),
    };

    log(&format!("Updating user username ({})", user_id));

    let result = diesel::update(users)
        .set(username.eq(new_username))
        .filter(id.eq(user_id))
        .execute(&connection);

    get_update_result(result, &connection, user_id)
}

/// Updates the description for an existing user in the user table
pub fn update_user_description(user_id: i32, new_description: &str) -> Result<User, Error> {
    use super::schema::users::dsl::{description, id, users};

    let connection = match establish_connection() {
        Some(con) => con,
        None => return Err(Error::Connection),
    };

    log(&format!("Updating user description ({})", user_id));

    let result = diesel::update(users)
        .set(description.eq(new_description))
        .filter(id.eq(user_id))
        .execute(&connection);

    get_update_result(result, &connection, user_id)
}

/// Updates the hidden flag for an existing user in the user table
pub fn update_user_avatar(user_id: i32, new_avatar: &str) -> Result<User, Error> {
    use super::schema::users::dsl::{avatar, id, users};

    let connection = match establish_connection() {
        Some(con) => con,
        None => return Err(Error::Connection),
    };

    log(&format!("Updating user avatar flag ({})", user_id));

    let result = diesel::update(users)
        .set(avatar.eq(new_avatar))
        .filter(id.eq(user_id))
        .execute(&connection);

    get_update_result(result, &connection, user_id)
}

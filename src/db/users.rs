use diesel::prelude::*;
use failure::ResultExt;

use super::DbConn;
use crate::types::User;
use crate::{IntErrorKind, IntResult};

use datatypes::valid::fields::*;
use datatypes::valid::ids::*;

/// Inserts new user into the user table
pub fn insert_user(connection: &DbConn, user_id: &UserId, user_name: &Username) -> IntResult<User> {
    use super::schema::users::dsl::{id, username, users};

    trace!("Inserting user ({:?}:{})", user_id, user_name);

    diesel::insert_into(users)
        .values((id.eq(*(*user_id)), username.eq(user_name.get_string())))
        .execute(connection)
        .context(IntErrorKind::QueryError)
        .and_then(|_| {
            users
                .filter(id.eq(*(*user_id)))
                .first::<User>(connection)
                .context(IntErrorKind::ContentNotFound)
        }).map_err(|e| e.into())
}

/// Gets an exisiting user from the user table
pub fn get_user(connection: &DbConn, user_id: &UserId) -> IntResult<User> {
    use super::schema::users::dsl::{id, users};

    trace!("Getting user ({:?})", user_id);

    users
        .filter(id.eq(*(*user_id)))
        .first::<User>(connection)
        .optional()
        .context(IntErrorKind::QueryError)?
        .ok_or(IntErrorKind::ContentNotFound)
        .map_err(|e| e.into())
}

/// Deletes an existing user from the user table
pub fn delete_user(connection: &DbConn, user_id: &UserId) -> IntResult<usize> {
    use super::schema::users::dsl::{id, users};

    trace!("Deleting user ({:?})", user_id);

    let num_deleted = diesel::delete(users)
        .filter(id.eq(*(*user_id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_deleted == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        Ok(num_deleted)
    }
}

/// Clears the user table
pub fn delete_all_users(connection: &DbConn) -> IntResult<usize> {
    use super::schema::users::dsl::users;

    trace!("Deleting all users");

    diesel::delete(users)
        .execute(connection)
        .context(IntErrorKind::QueryError)
        .map_err(|e| e.into())
}
/*
/// Updates an existing user in the user table
pub fn update_user(connection: &DbConn, user: &User) -> IntResult<User> {
    use super::schema::users::dsl::{id, users};

    trace!("Updating user ({}:{})", user.id, user.username);

    let num_updated = diesel::update(users)
        .set(user)
        .filter(id.eq(user.id))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_user(connection, user.id)
    }
}
*/

/// Updates the title for an existing user in the user table
pub fn update_user_username(
    connection: &DbConn,
    user_id: &UserId,
    new_username: &Username,
) -> IntResult<User> {
    use super::schema::users::dsl::{id, username, users};

    trace!("Updating user username ({:?})", user_id);

    let num_updated = diesel::update(users)
        .set(username.eq(new_username.get_string()))
        .filter(id.eq(*(*user_id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_user(connection, user_id)
    }
}

/// Updates the description for an existing user in the user table
pub fn update_user_description(
    connection: &DbConn,
    user_id: &UserId,
    new_description: &Description,
) -> IntResult<User> {
    use super::schema::users::dsl::{description, id, users};

    trace!("Updating user description ({:?})", user_id);

    let num_updated = diesel::update(users)
        .set(description.eq(new_description.get_string()))
        .filter(id.eq(*(*user_id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_user(connection, user_id)
    }
}

/// Updates the hidden flag for an existing user in the user table
pub fn update_user_avatar(
    connection: &DbConn,
    user_id: &UserId,
    new_avatar: &str,
) -> IntResult<User> {
    use super::schema::users::dsl::{avatar, id, users};

    trace!("Updating user avatar flag ({:?})", user_id);

    let num_updated = diesel::update(users)
        .set(avatar.eq(new_avatar))
        .filter(id.eq(*(*user_id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_user(connection, user_id)
    }
}

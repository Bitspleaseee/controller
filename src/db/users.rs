use diesel::prelude::*;
use failure::ResultExt;

use super::DbConn;
use crate::types::User;
use crate::{IntErrorKind, IntResult};

use datatypes::valid::fields::*;
use datatypes::valid::ids::*;

/// Inserts new user into the user table
pub fn insert_user(connection: &DbConn, id: &UserId, username: &Username) -> IntResult<User> {
    use super::schema::users::dsl;

    trace!("Inserting user ({:?}:{})", id, username);

    diesel::insert_into(dsl::users)
        .values((dsl::id.eq(*(*id)), dsl::username.eq(username.as_ref())))
        .execute(connection)
        .context(IntErrorKind::QueryError)
        .and_then(|_| {
            dsl::users
                .filter(dsl::id.eq(*(*id)))
                .first::<User>(connection)
                .context(IntErrorKind::ContentNotFound)
        }).map_err(|e| e.into())
}

/// Gets an exisiting user from the user table
pub fn get_user(connection: &DbConn, user_id: &UserId) -> IntResult<User> {
    use super::schema::users::dsl;

    trace!("Getting user with id ({})", user_id);

    dsl::users
        .filter(dsl::id.eq(*(*user_id)))
        .first::<User>(connection)
        .optional()
        .context(IntErrorKind::QueryError)?
        .ok_or(IntErrorKind::ContentNotFound)
        .map(|u| {
            trace!("Found user with id ({}: {})", user_id, &u.username);
            u
        }).map_err(|e| {
            trace!("Did not find user with id ({})", user_id);
            e.into()
        })
}

/// Deletes an existing user from the user table
pub fn delete_user(connection: &DbConn, id: &UserId) -> IntResult<usize> {
    use super::schema::users::dsl;

    trace!("Deleting user ({:?})", id);

    let num_deleted = diesel::delete(dsl::users)
        .filter(dsl::id.eq(*(*id)))
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
    use super::schema::users::dsl;

    trace!("Deleting all users");

    diesel::delete(dsl::users)
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
    id: &UserId,
    username: &Username,
) -> IntResult<User> {
    use super::schema::users::dsl;

    trace!("Updating user username ({:?})", id);

    let num_updated = diesel::update(dsl::users)
        .set(dsl::username.eq(username.as_ref()))
        .filter(dsl::id.eq(*(*id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_user(connection, id)
    }
}

/// Updates the description for an existing user in the user table
pub fn update_user_description(
    connection: &DbConn,
    id: &UserId,
    description: &Description,
) -> IntResult<User> {
    use super::schema::users::dsl;

    trace!("Updating user description ({:?})", id);

    let num_updated = diesel::update(dsl::users)
        .set(dsl::description.eq(description.as_ref()))
        .filter(dsl::id.eq(*(*id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_user(connection, id)
    }
}

/// Updates the hidden flag for an existing user in the user table
pub fn update_user_avatar(connection: &DbConn, id: &UserId, avatar: &str) -> IntResult<User> {
    use super::schema::users::dsl;

    trace!("Updating user avatar flag ({:?})", id);

    let num_updated = diesel::update(dsl::users)
        .set(dsl::avatar.eq(avatar))
        .filter(dsl::id.eq(*(*id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_user(connection, id)
    }
}

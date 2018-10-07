use diesel::prelude::*;
use failure::ResultExt;

use super::DbConn;
use crate::types::{InsertUser, UpdateUser, User};
use crate::{IntErrorKind, IntResult};

use datatypes::valid::ids::*;

/// Inserts new user into the user table
///
/// TODO change this implementation to make the user-id auto increment
pub fn insert_user(con: &DbConn, user: impl Into<InsertUser>) -> IntResult<User> {
    use super::schema::users::dsl;
    let user = user.into();

    trace!("Inserting user");

    user.insert_into(dsl::users)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to insert user: {:?}", e);
            e.into()
        }).and_then(|_| {
            dsl::users
                .order(dsl::id.desc())
                .first::<User>(con)
                .optional()
                .context(IntErrorKind::QueryError)?
                .ok_or(IntErrorKind::ContentNotFound)
                .map_err(|e| {
                    error!("Unable to get user after insertion: {}", e);
                    e.into()
                })
        })
}

/// Gets an exisiting user from the user table
pub fn get_user(con: &DbConn, id: UserId) -> IntResult<User> {
    use super::schema::users::dsl;

    trace!("Getting user ({})", id);

    dsl::users
        .filter(dsl::id.eq(*id))
        .first::<User>(con)
        .optional()
        .context(IntErrorKind::QueryError)?
        .ok_or(IntErrorKind::ContentNotFound)
        .map_err(|e| {
            trace!("Unable to get user ({}): {}", id, e);
            e.into()
        })
}

/// Deletes an existing user from the user table
pub fn delete_user(con: &DbConn, id: UserId) -> IntResult<usize> {
    use super::schema::users::dsl;

    trace!("Deleting user ({})", id);

    let num_deleted = diesel::delete(dsl::users)
        .filter(dsl::id.eq(*id))
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to delete user ({}): {}", id, e);
            e
        })?;

    if num_deleted == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        Ok(num_deleted)
    }
}

/// Clears the user table
pub fn delete_all_users(con: &DbConn) -> IntResult<usize> {
    use super::schema::users::dsl;

    trace!("Deleting all users");

    diesel::delete(dsl::users)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to delete all users: {}", e);
            e.into()
        })
}

/// Updates an existing user in the user table
pub fn update_user(con: &DbConn, user: impl Into<UpdateUser>) -> IntResult<User> {
    let user = user.into();
    let id = user.id;

    trace!("Updating user ({})", id);

    user.save_changes(con)
        .optional()
        .context(IntErrorKind::QueryError)?
        .ok_or(IntErrorKind::ContentNotFound)
        .map_err(|e| {
            error!("Unable to update user ({}): {}", id, e);
            e.into()
        })
}

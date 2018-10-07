use diesel::prelude::*;
use failure::ResultExt;

use super::DbConn;
use crate::types::{InsertUser, UpdateUser, User};
use crate::{IntErrorKind, IntResult};

use datatypes::valid::ids::*;

/// Inserts new user into the user table
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::establish_connection;

    #[test]
    fn insert_and_get() {
        let con = establish_connection(&std::env::var("DATABASE_URL").unwrap()).unwrap();
        assert!(delete_all_users(&con).is_ok());

        let insert_data = InsertUser {
            id: 1,
            username: "TestUser".to_string(),
        };

        let expected_data = User {
            id: 1,
            username: "TestUser".to_string(),
            description: None,
            avatar: None,
        };

        // Insert
        let returned_data = insert_user(&con, insert_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Compare
        assert_eq!(returned_data, expected_data);

        // Get
        let returned_data = get_user(&con, 1.into());
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Compare
        assert_eq!(returned_data, expected_data);
    }

    #[test]
    fn update() {
        let con = establish_connection(&std::env::var("DATABASE_URL").unwrap()).unwrap();
        assert!(delete_all_users(&con).is_ok());

        let insert_data = InsertUser {
            id: 1,
            username: "TestUser".to_string(),
        };

        let update_data = UpdateUser {
            id: 1,
            description: Some("TestDescription".to_string()),
            avatar: Some("TestAvatar".to_string()),
        };

        let expected_data = User {
            id: 1,
            username: "TestUser".to_string(),
            description: Some("TestDescription".to_string()),
            avatar: Some("TestAvatar".to_string()),
        };

        // Insert
        assert!(insert_user(&con, insert_data).is_ok());

        // Update
        let returned_data = update_user(&con, update_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Compare
        assert_eq!(returned_data, expected_data);
    }

    #[test]
    fn delete() {
        let con = establish_connection(&std::env::var("DATABASE_URL").unwrap()).unwrap();
        assert!(delete_all_users(&con).is_ok());

        // insert
        let insert_data = InsertUser {
            id: 1,
            username: "TestUser".to_string(),
        };
        assert!(insert_user(&con, insert_data).is_ok());

        // Get
        assert!(get_user(&con, 1.into()).is_ok());

        // Delete
        assert!(delete_user(&con, 1.into()).is_ok());

        // Fail to get
        assert!(get_user(&con, 1.into()).is_err());
    }
}

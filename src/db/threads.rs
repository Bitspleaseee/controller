use diesel::prelude::*;
use failure::ResultExt;

use super::{DbConn, MAX_THREAD_LIMIT};
use crate::types::{InsertThread, Thread, UpdateThread};
use crate::{IntErrorKind, IntResult};

use datatypes::valid::ids::*;

/// Inserts a new thread into the thread table
pub fn insert_thread(con: &DbConn, thread: impl Into<InsertThread>) -> IntResult<Thread> {
    use super::schema::threads::dsl;
    let thread = thread.into();

    trace!("Inserting thread");

    thread
        .insert_into(dsl::threads)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to insert new thread: {:?}", e);
            e.into()
        }).and_then(|_| {
            // TODO can we be sure that this is the same thread that was
            // inserted? If connections to the database happen concurrently,
            // a different thread might insert a new thread between the time we
            // inserted our thread and got the thread with the highest id back
            dsl::threads
                .order(dsl::id.desc())
                .first(con)
                .optional()
                .context(IntErrorKind::QueryError)?
                .ok_or(IntErrorKind::ContentNotFound)
                .map_err(|e| {
                    error!("Unable to get thread after insertion: {}", e);
                    e.into()
                })
        })
}

/// Gets an exisiting thread from the thread table
pub fn get_thread(con: &DbConn, id: ThreadId, include_hidden: bool) -> IntResult<Thread> {
    use super::schema::threads::dsl;

    trace!("Getting thread ({}) [{}]", id, fmt_hidden!(include_hidden));

    if include_hidden {
        dsl::threads.filter(dsl::id.eq(*id)).first::<Thread>(con)
    } else {
        dsl::threads
            .filter(dsl::id.eq(*id))
            .filter(dsl::hidden.eq(false))
            .first::<Thread>(con)
    }.optional()
    .context(IntErrorKind::QueryError)?
    .ok_or(IntErrorKind::ContentNotFound)
    .map_err(|e| {
        error!("Unable to get thread ({}): {}", id, e);
        e.into()
    })
}

/// Gets all the threads from the thread table
pub fn get_all_threads(con: &DbConn, include_hidden: bool) -> IntResult<Vec<Thread>> {
    use super::schema::threads::dsl;

    trace!("Getting all threads [{}]", fmt_hidden!(include_hidden));

    if include_hidden {
        dsl::threads.limit(MAX_THREAD_LIMIT).get_results(con)
    } else {
        dsl::threads
            .limit(MAX_THREAD_LIMIT)
            .filter(dsl::hidden.eq(false))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to get all threads: {}", e);
        e.into()
    })
}

/// Gets all the comments in a thread from the comment table
pub fn get_threads_in_category(
    con: &DbConn,
    category_id: CategoryId,
    include_hidden: bool,
) -> IntResult<Vec<Thread>> {
    use super::schema::threads::dsl;

    trace!("Getting threads in category ({})", category_id);

    if include_hidden {
        dsl::threads
            .limit(MAX_THREAD_LIMIT)
            .filter(dsl::category_id.eq(*category_id))
            .get_results(con)
    } else {
        dsl::threads
            .limit(MAX_THREAD_LIMIT)
            .filter(dsl::category_id.eq(*category_id))
            .filter(dsl::hidden.eq(false))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to get threads in category ({}): {}", category_id, e);
        e.into()
    })
}

/// Clears the thread table
pub fn delete_all_threads(con: &DbConn) -> IntResult<usize> {
    use super::schema::threads::dsl;

    trace!("Deleting all threads");

    diesel::delete(dsl::threads)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to delete all threads: {}", e);
            e.into()
        })
}

/// Updates an existing thread in the thread table
pub fn update_thread(con: &DbConn, thread: impl Into<UpdateThread>) -> IntResult<Thread> {
    let thread = thread.into();
    let id = thread.id;

    trace!("Updating thread ({})", id);

    thread
        .save_changes(con)
        .optional()
        .context(IntErrorKind::QueryError)?
        .ok_or(IntErrorKind::ContentNotFound)
        .map_err(|e| {
            error!("Unable to update thread ({}): {}", id, e);
            e.into()
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use crate::db::{categories, establish_connection, users};
    use crate::types::{InsertCategory, InsertUser};

    #[test]
    fn insert_and_get() {
        let con = establish_connection(&std::env::var("DATABASE_URL").unwrap()).unwrap();

        // User
        let insert_data = InsertUser {
            id: 10,
            username: "TestUser".to_string(),
        };
        let returned_data = users::insert_user(&con, insert_data);
        assert!(returned_data.is_ok());
        let user = returned_data.unwrap();

        // Category
        let insert_data = InsertCategory {
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };
        let returned_data = categories::insert_category(&con, insert_data);
        assert!(returned_data.is_ok());
        let category = returned_data.unwrap();

        // Thread
        let insert_data = InsertThread {
            category_id: category.id,
            user_id: user.id,
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };

        let mut expected_data = Thread {
            id: 1,
            category_id: category.id,
            user_id: user.id,
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
            timestamp: NaiveDateTime::from_timestamp(0, 0),
            hidden: false,
        };

        // Insert
        let returned_data = insert_thread(&con, insert_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Compare
        expected_data.id = returned_data.id;
        expected_data.timestamp = returned_data.timestamp;
        assert_eq!(returned_data, expected_data);

        // Get
        let returned_data = get_thread(&con, returned_data.id.into(), false);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Compare
        assert_eq!(returned_data, expected_data);
    }

    #[test]
    fn update() {
        let con = establish_connection(&std::env::var("DATABASE_URL").unwrap()).unwrap();

        // User
        let insert_data = InsertUser {
            id: 11,
            username: "TestUser".to_string(),
        };
        let returned_data = users::insert_user(&con, insert_data);
        assert!(returned_data.is_ok());
        let user = returned_data.unwrap();

        // Category
        let insert_data = InsertCategory {
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };
        let returned_data = categories::insert_category(&con, insert_data);
        assert!(returned_data.is_ok());
        let category = returned_data.unwrap();

        // Thread
        let insert_data = InsertThread {
            category_id: category.id,
            user_id: user.id,
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };

        let mut update_data = UpdateThread {
            id: 1,
            title: Some("OtherTitle".to_string()),
            description: Some("OtherDescription".to_string()),
            hidden: Some(true),
        };

        let mut expected_data = Thread {
            id: 1,
            category_id: category.id,
            user_id: user.id,
            title: "OtherTitle".to_string(),
            description: "OtherDescription".to_string(),
            timestamp: NaiveDateTime::from_timestamp(0, 0),
            hidden: true,
        };

        // Insert
        let returned_data = insert_thread(&con, insert_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Update
        update_data.id = returned_data.id;
        let returned_data = update_thread(&con, update_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Compare
        expected_data.id = returned_data.id;
        expected_data.timestamp = returned_data.timestamp;
        assert_eq!(returned_data, expected_data);
    }

    #[test]
    fn hide() {
        let con = establish_connection(&std::env::var("DATABASE_URL").unwrap()).unwrap();

        // User
        let insert_data = InsertUser {
            id: 12,
            username: "TestUser".to_string(),
        };
        let returned_data = users::insert_user(&con, insert_data);
        assert!(returned_data.is_ok());
        let user = returned_data.unwrap();

        // Category
        let insert_data = InsertCategory {
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };
        let returned_data = categories::insert_category(&con, insert_data);
        assert!(returned_data.is_ok());
        let category = returned_data.unwrap();

        // Thread
        let insert_data = InsertThread {
            category_id: category.id,
            user_id: user.id,
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };

        let mut update_data = UpdateThread {
            id: 1,
            title: None,
            description: None,
            hidden: Some(true),
        };

        // insert
        let returned_data = insert_thread(&con, insert_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Get
        assert!(get_thread(&con, returned_data.id.into(), false).is_ok());

        // Delete
        update_data.id = returned_data.id;
        assert!(update_thread(&con, update_data).is_ok());

        // Fail to get
        assert!(get_thread(&con, returned_data.id.into(), false).is_err());
    }
}

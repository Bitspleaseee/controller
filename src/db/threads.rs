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

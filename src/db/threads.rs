use diesel::prelude::*;
use failure::ResultExt;

use super::DbConn;
use crate::types::Thread;
use crate::{IntErrorKind, IntResult};

use datatypes::valid::fields::*;
use datatypes::valid::ids::*;

const MAX_THREAD_LIMIT: i64 = 30;

/// Simple macro to format a hidden bool
macro_rules! fmt_hidden {
    ($hidden:expr) => {
        if $hidden {
            "including hidden"
        } else {
            "excluding hidden"
        }
    };
}

/// Inserts a new thread into the thread table
pub fn insert_thread(
    connection: &DbConn,
    title: &Title,
    description: &Description,
) -> IntResult<Thread> {
    use super::schema::threads::dsl;

    trace!("Inserting thread");

    diesel::insert_into(dsl::threads)
        .values((
            dsl::title.eq(title.as_ref()),
            dsl::description.eq(description.as_ref()),
        )).execute(connection)
        .context(IntErrorKind::QueryError)
        .and_then(|_| {
            dsl::threads
                .order(dsl::id.desc())
                .first(connection)
                .context(IntErrorKind::ContentNotFound)
        }).map(|t: Thread| {
            trace!("Inserted thread got id {}", t.id);
            t
        }).map_err(|e| {
            error!("Unable to insert new thread: {}", e);
            e.into()
        })
}

/// Gets an exisiting thread from the thread table
pub fn get_thread(connection: &DbConn, id: &ThreadId, include_hidden: bool) -> IntResult<Thread> {
    use super::schema::threads::dsl;

    trace!(
        "Getting thread with id {:?} ({})",
        id,
        fmt_hidden!(include_hidden)
    );

    if include_hidden {
        dsl::threads
            .filter(dsl::id.eq(*(*id)))
            .first::<Thread>(connection)
            .optional()
    } else {
        dsl::threads
            .filter(dsl::id.eq(*(*id)))
            .filter(dsl::hidden.eq(false))
            .first::<Thread>(connection)
            .optional()
    }.context(IntErrorKind::QueryError)?
    .ok_or(IntErrorKind::ContentNotFound)
    .map_err(|e| {
        error!("Unable to get thread with id {}: {}", id, e);
        e.into()
    })
}

/// Gets all the threads from the thread table
pub fn get_all_threads(connection: &DbConn, include_hidden: bool) -> IntResult<Vec<Thread>> {
    use super::schema::threads::dsl;

    trace!("Getting all threads ({})", fmt_hidden!(include_hidden));

    if include_hidden {
        dsl::threads.limit(MAX_THREAD_LIMIT).get_results(connection)
    } else {
        dsl::threads
            .limit(MAX_THREAD_LIMIT)
            .filter(dsl::hidden.eq(false))
            .get_results(connection)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to get all threads: {}", e);
        e.into()
    })
}

/// Gets all the comments in a thread from the comment table
pub fn get_threads_in_category(
    connection: &DbConn,
    category_id: &CategoryId,
    include_hidden: bool,
) -> IntResult<Vec<Thread>> {
    use super::schema::threads::dsl;

    trace!("Getting threads in category with id {}", category_id);

    if include_hidden {
        dsl::threads
            .limit(MAX_THREAD_LIMIT)
            .filter(dsl::category_id.eq(*(*category_id)))
            .get_results(connection)
    } else {
        dsl::threads
            .limit(MAX_THREAD_LIMIT)
            .filter(dsl::category_id.eq(*(*category_id)))
            .filter(dsl::hidden.eq(false))
            .get_results(connection)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!(
            "Unable to get threads in category with id {}: {}",
            category_id, e
        );
        e.into()
    })
}

/// Clears the thread table
pub fn delete_all_threads(connection: &DbConn) -> IntResult<usize> {
    use super::schema::threads::dsl;

    trace!("Deleting all threads");

    diesel::delete(dsl::threads)
        .execute(connection)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to delete all threads: {}", e);
            e.into()
        })
}

/// Updates an existing thread in the thread table
pub fn update_thread(
    connection: &DbConn,
    id: &ThreadId,
    title: &Title,
    description: &Description,
) -> IntResult<Thread> {
    use super::schema::threads::dsl;

    trace!("Updating thread ({:?})", id);

    let num_updated = diesel::update(dsl::threads)
        .set((
            dsl::title.eq(title.as_ref()),
            dsl::description.eq(description.as_ref()),
        )).filter(dsl::id.eq(*(*id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_thread(connection, id, false)
    }
}

/// Updates the hidden flag for an existing thread in the thread table
pub fn update_thread_hidden(connection: &DbConn, id: &ThreadId, hidden: bool) -> IntResult<Thread> {
    use super::schema::threads::dsl;

    trace!("Updating thread hidden flag ({:?})", id);

    let num_updated = diesel::update(dsl::threads)
        .set(dsl::hidden.eq(hidden))
        .filter(dsl::id.eq(*(*id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_thread(&connection, id, true)
    }
}

use diesel::prelude::*;
use failure::ResultExt;

use super::DbConn;
use crate::types::Thread;
use crate::{IntErrorKind, IntResult};

use datatypes::valid::fields::*;
use datatypes::valid::ids::*;

/// Inserts a new thread into the thread table
pub fn insert_thread(
    connection: &DbConn,
    title: &Title,
    description: &Description,
) -> IntResult<Thread> {
    use super::schema::threads::dsl;

    trace!("Inserting thread: {}", title);
    Err(IntErrorKind::ServerError)?

    //diesel::insert_into(dsl::threads)
    //    .values((
    //        dsl::title.eq(title.as_ref()),
    //        dsl::description.eq(description.as_ref()),
    //    )).execute(connection)
    //    .context(IntErrorKind::QueryError)
    //    .and_then(|_| {
    //        threads
    //            .order(dsl::id.desc())
    //            .first(connection)
    //            .context(IntErrorKind::ContentNotFound)
    //    }).map_err(|e| e.into())
}

/// Gets an exisiting thread from the thread table
fn get_thread(connection: &DbConn, id: &ThreadId, include_hidden: bool) -> IntResult<Thread> {
    use super::schema::threads::dsl;

    trace!("Getting thread ({:?})", id);
    Err(IntErrorKind::ServerError)?

    //dsl::threads
    //    .filter(dsl::id.eq(*(*id)))
    //    .first::<Thread>(connection)
    //    .optional()
    //    .context(IntErrorKind::QueryError)?
    //    .ok_or(IntErrorKind::ContentNotFound)
    //    .map_err(|e| e.into())
}

/// Gets all the threads from the thread table
pub fn get_all_threads(connection: &DbConn, include_hidden: bool) -> IntResult<Vec<Thread>> {
    use super::schema::threads::dsl;
    
    trace!("Getting all threads, include hidden: {}", include_hidden);
    Err(IntErrorKind::ServerError)?

    //if include_hidden {
    //    threads.get_results(connection)
    //} else {
    //    threads.filter(hidden.eq(false)).get_results(connection)
    //}.context(IntErrorKind::QueryError)
    //.map_err(|e| e.into())
}

/// Clears the thread table
pub fn delete_all_threads(connection: &DbConn) -> IntResult<usize> {
    use super::schema::threads::dsl;

    trace!("Deleting all threads");

    diesel::delete(dsl::threads)
        .execute(connection)
        .context(IntErrorKind::QueryError)
        .map_err(|e| e.into())
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
        //get_thread(&connection, )
        Err(IntErrorKind::ContentNotFound)?
    }
}

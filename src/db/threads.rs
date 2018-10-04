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
    new_title: &Title,
    new_description: &Description,
) -> IntResult<Thread> {
    use super::schema::threads::dsl::{description, id, threads, title};

    trace!("Inserting thread: {}", new_title);
    unimplemented!();

    //diesel::insert_into(threads)
    //    .values((
    //        title.eq(new_title.as_ref()),
    //        description.eq(new_description.as_ref()),
    //    )).execute(connection)
    //    .context(IntErrorKind::QueryError)
    //    .and_then(|_| {
    //        threads
    //            .order(id.desc())
    //            .first(connection)
    //            .context(IntErrorKind::ContentNotFound)
    //    }).map_err(|e| e.into())
}

/// Gets an exisiting thread from the thread table
fn get_thread(connection: &DbConn, thread_id: &ThreadId) -> IntResult<Thread> {
    use super::schema::threads::dsl::{id, threads};

    trace!("Getting thread ({:?})", thread_id);
    unimplemented!();

    //threads
    //    .filter(id.eq(*(*thread_id)))
    //    .first::<Thread>(connection)
    //    .optional()
    //    .context(IntErrorKind::QueryError)?
    //    .ok_or(IntErrorKind::ContentNotFound)
    //    .map_err(|e| e.into())
}

/// Gets all the threads from the thread table
pub fn get_all_threads(connection: &DbConn, include_hidden: bool) -> IntResult<Vec<Thread>> {
    use super::schema::threads::dsl::{hidden, threads};

    trace!("Getting all threads, include hidden: {}", include_hidden);
    unimplemented!();

    //if include_hidden {
    //    threads.get_results(connection)
    //} else {
    //    threads.filter(hidden.eq(false)).get_results(connection)
    //}.context(IntErrorKind::QueryError)
    //.map_err(|e| e.into())
}

/// Clears the thread table
pub fn delete_all_threads(connection: &DbConn) -> IntResult<usize> {
    use super::schema::threads::dsl::threads;

    trace!("Deleting all threads");

    diesel::delete(threads)
        .execute(connection)
        .context(IntErrorKind::QueryError)
        .map_err(|e| e.into())
}

/// Updates an existing thread in the thread table
pub fn update_thread(
    connection: &DbConn,
    thread_id: &ThreadId,
    new_title: &Title,
    new_description: &Description,
) -> IntResult<Thread> {
    use super::schema::threads::dsl::{description, id, threads, title};

    trace!("Updating thread ({:?})", thread_id);

    let num_updated = diesel::update(threads)
        .set((
            title.eq(new_title.as_ref()),
            description.eq(new_description.as_ref()),
        )).filter(id.eq(*(*thread_id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        //get_thread(&connection, )
        Err(IntErrorKind::ContentNotFound)?
    }
}

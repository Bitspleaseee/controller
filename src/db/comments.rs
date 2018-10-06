use diesel::prelude::*;
use failure::ResultExt;

use super::DbConn;
use crate::types::Comment;
use crate::{IntErrorKind, IntResult};

use datatypes::valid::fields::*;
use datatypes::valid::ids::*;

/// Inserts a new comment into the comment table
pub fn insert_comment(
    connection: &DbConn,
    thread_id: &ThreadId,
    user_id: &UserId,
    parent_id: &Option<CommentId>,
    content: &CommentContent,
) -> IntResult<Comment> {
    use super::schema::comments::dsl;

    trace!("Inserting comment");

    let parent_id: Option<u32> = parent_id.map(|pid| *pid);

    diesel::insert_into(dsl::comments)
        .values((
            dsl::thread_id.eq(*(*thread_id)),
            dsl::user_id.eq(*(*user_id)),
            dsl::parent_id.eq(parent_id),
            dsl::content.eq(content.as_ref()),
        )).execute(connection)
        .context(IntErrorKind::QueryError)
        .and_then(|_| {
            dsl::comments
                .order(dsl::id.desc())
                .first(connection)
                .context(IntErrorKind::ContentNotFound)
        }).map_err(|e| e.into())
}

/// Gets an exisiting comment from the comment table
pub fn get_comment(
    connection: &DbConn,
    comment_id: &CommentId,
    include_hidden: bool,
) -> IntResult<Comment> {
    use super::schema::comments::dsl;

    trace!("Getting comment ({:?})", comment_id);

    if include_hidden {
        dsl::comments
            .filter(dsl::id.eq(*(*comment_id)))
            .first::<Comment>(connection)
    } else {
        dsl::comments
            .filter(dsl::id.eq(*(*comment_id)))
            .filter(dsl::hidden.eq(false))
            .first::<Comment>(connection)
    }.optional()
    .context(IntErrorKind::QueryError)?
    .ok_or(IntErrorKind::ContentNotFound)
    .map_err(|e| e.into())
}

/// Gets all the comments from the comment table
pub fn get_all_comments(connection: &DbConn, include_hidden: bool) -> IntResult<Vec<Comment>> {
    use super::schema::comments::dsl;

    trace!("Getting all comments, include hidden: {}", include_hidden);

    if include_hidden {
        dsl::comments.get_results(connection)
    } else {
        dsl::comments
            .filter(dsl::hidden.eq(false))
            .get_results(connection)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| e.into())
}

/// Gets all the comments in a thread from the comment table
pub fn get_comments_in_thread(
    connection: &DbConn,
    thread_id: &ThreadId,
    include_hidden: bool,
) -> IntResult<Vec<Comment>> {
    use super::schema::comments::dsl;

    trace!("Getting comments in thread: ({:?})", thread_id);

    if include_hidden {
        dsl::comments
            .filter(dsl::thread_id.eq(*(*thread_id)))
            .get_results(connection)
    } else {
        dsl::comments
            .filter(dsl::thread_id.eq(*(*thread_id)))
            .filter(dsl::hidden.eq(false))
            .get_results(connection)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| e.into())
}

/// Clears the comment table
pub fn delete_all_comments(connection: &DbConn) -> IntResult<usize> {
    use super::schema::comments::dsl;

    trace!("Deleting all comments");

    diesel::delete(dsl::comments)
        .execute(connection)
        .context(IntErrorKind::QueryError)
        .map_err(|e| e.into())
}

/// Updates an existing comment in the comment table
pub fn update_comment(
    connection: &DbConn,
    id: &CommentId,
    content: &CommentContent,
) -> IntResult<Comment> {
    use super::schema::comments::dsl;

    trace!("Updating comment ({:?})", id);

    let num_updated = diesel::update(dsl::comments)
        .set(dsl::content.eq(content.as_ref()))
        .filter(dsl::id.eq(*(*id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_comment(&connection, id, true)
    }
}

/// Updates the hidden flag for an existing comment in the comment table
pub fn update_comment_hidden(
    connection: &DbConn,
    id: &CommentId,
    hidden: bool,
) -> IntResult<Comment> {
    use super::schema::comments::dsl;

    trace!("Updating comment hidden flag ({:?})", id);

    let num_updated = diesel::update(dsl::comments)
        .set(dsl::hidden.eq(hidden))
        .filter(dsl::id.eq(*(*id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_comment(&connection, id, true)
    }
}

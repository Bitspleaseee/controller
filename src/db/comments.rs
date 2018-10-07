use diesel::prelude::*;
use failure::ResultExt;

use super::{DbConn, MAX_COMMENT_LIMIT};
use crate::types::{Comment, InsertComment, UpdateComment};
use crate::{IntErrorKind, IntResult};

use datatypes::valid::ids::*;

/// Inserts a new comment into the comment table
pub fn insert_comment(con: &DbConn, comment: impl Into<InsertComment>) -> IntResult<Comment> {
    use super::schema::comments::dsl;
    let comment = comment.into();

    trace!("Inserting comment");

    comment
        .insert_into(dsl::comments)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to insert comment: {}", e);
            e.into()
        }).and_then(|_| {
            dsl::comments
                .order(dsl::id.desc())
                .first(con)
                .optional()
                .context(IntErrorKind::QueryError)?
                .ok_or(IntErrorKind::ContentNotFound)
                .map_err(|e| {
                    error!("Unable to get comment after insertion: {}", e);
                    e.into()
                })
        })
}

/// Gets an exisiting comment from the comment table
pub fn get_comment(con: &DbConn, id: CommentId, include_hidden: bool) -> IntResult<Comment> {
    use super::schema::comments::dsl;

    trace!("Getting comment ({})", id);

    if include_hidden {
        dsl::comments.filter(dsl::id.eq(*id)).first::<Comment>(con)
    } else {
        dsl::comments
            .filter(dsl::id.eq(*id))
            .filter(dsl::hidden.eq(false))
            .first::<Comment>(con)
    }.optional()
    .context(IntErrorKind::QueryError)?
    .ok_or(IntErrorKind::ContentNotFound)
    .map_err(|e| {
        error!("Unable to get comment ({}): {}", id, e);
        e.into()
    })
}

/// Gets all the comments from the comment table
pub fn get_all_comments(con: &DbConn, include_hidden: bool) -> IntResult<Vec<Comment>> {
    use super::schema::comments::dsl;

    trace!("Getting all comments, include hidden: {}", include_hidden);

    if include_hidden {
        dsl::comments.limit(MAX_COMMENT_LIMIT).get_results(con)
    } else {
        dsl::comments
            .limit(MAX_COMMENT_LIMIT)
            .filter(dsl::hidden.eq(false))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to get comments: {}", e);
        e.into()
    })
}

/// Gets all the comments in a thread from the comment table
pub fn get_comments_in_thread(
    con: &DbConn,
    id: ThreadId,
    include_hidden: bool,
) -> IntResult<Vec<Comment>> {
    use super::schema::comments::dsl;

    trace!("Getting comments in thread ({})", id);

    if include_hidden {
        dsl::comments
            .limit(MAX_COMMENT_LIMIT)
            .filter(dsl::id.eq(*id))
            .get_results(con)
    } else {
        dsl::comments
            .limit(MAX_COMMENT_LIMIT)
            .filter(dsl::id.eq(*id))
            .filter(dsl::hidden.eq(false))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to get comments in thread ({}): {}", id, e);
        e.into()
    })
}

/// Clears the comment table
pub fn delete_all_comments(con: &DbConn) -> IntResult<usize> {
    use super::schema::comments::dsl;

    trace!("Deleting all comments");

    diesel::delete(dsl::comments)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to delete all comments: {}", e);
            e.into()
        })
}

/// Updates an existing comment in the comment table
pub fn update_comment(con: &DbConn, comment: impl Into<UpdateComment>) -> IntResult<Comment> {
    let comment = comment.into();
    let id = comment.id;

    trace!("Updating comment ({})", id);

    comment
        .save_changes(con)
        .optional()
        .context(IntErrorKind::QueryError)?
        .ok_or(IntErrorKind::ContentNotFound)
        .map_err(|e| {
            error!("Unable to update comment ({}): {}", id, e);
            e.into()
        })
}

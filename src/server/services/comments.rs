use failure::ResultExt;
use std::convert::TryInto;

use datatypes::content::requests::*;
use datatypes::content::responses::*;

use crate::db::{self, DbConn};
use crate::types::Comment;
use crate::{IntErrorKind, IntResult};

pub fn get_comment(con: &DbConn, payload: GetCommentPayload) -> IntResult<CommentPayload> {
    let GetCommentPayload { id, include_hidden } = payload;
    trace!("get_comment: {:?}", payload);

    db::comments::get_comment(&con, id, include_hidden).and_then(|p| {
        <Comment as TryInto<CommentPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert comment ({}) to payload: {}", id, e);
                e.into()
            })
    })
}

pub fn get_comments_in_thread(
    con: &DbConn,
    payload: GetCommentsPayload,
) -> IntResult<Vec<CommentPayload>> {
    let GetCommentsPayload { id, include_hidden } = payload;
    trace!("get_comments_in_thread: {:?}", payload);

    db::comments::get_comments_in_thread(&con, id, include_hidden).and_then(|comments| {
        comments
            .into_iter()
            .map(|comment| comment.try_into())
            .collect::<Result<Vec<CommentPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert comment ({}) to payload: {}", id, e);
                e.into()
            })
    })
}

pub fn get_all_comments(con: &DbConn, payload: GetHiddenPayload) -> IntResult<Vec<CommentPayload>> {
    let GetHiddenPayload { include_hidden } = payload;
    trace!("get_all_comments: {:?}", payload);

    db::comments::get_all_comments(&con, include_hidden).and_then(|comments| {
        comments
            .into_iter()
            .map(|comment| comment.try_into())
            .collect::<Result<Vec<CommentPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert comment to payload: {}", e);
                e.into()
            })
    })
}

pub fn add_comment(con: &DbConn, payload: AddCommentPayload) -> IntResult<CommentPayload> {
    trace!("add_comment: {:?}", payload);

    db::comments::insert_comment(&con, payload).and_then(|p| {
        <Comment as TryInto<CommentPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert comment to payload: {}", e);
                e.into()
            })
    })
}

pub fn edit_comment(con: &DbConn, payload: EditCommentPayload) -> IntResult<CommentPayload> {
    trace!("edit_comment: {:?}", payload);

    db::comments::update_comment(&con, payload).and_then(|p| {
        <Comment as TryInto<CommentPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert comment to payload: {}", e);
                e.into()
            })
    })
}

pub fn hide_comment(con: &DbConn, payload: HideCommentPayload) -> IntResult<CommentPayload> {
    trace!("hide_comment: {:?}", payload);

    db::comments::update_comment(&con, payload).and_then(|p| {
        <Comment as TryInto<CommentPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert comment to payload: {}", e);
                e.into()
            })
    })
}

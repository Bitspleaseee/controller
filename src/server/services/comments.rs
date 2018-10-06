use failure::ResultExt;
use std::convert::TryInto;

use datatypes::content::requests::*;
use datatypes::content::responses::*;

use crate::db::{self, DbConn};
use crate::types::Comment;
use crate::{IntErrorKind, IntResult};

pub fn get_comment(con: &DbConn, payload: GetCommentPayload) -> IntResult<CommentPayload> {
    trace!("get_comment {:?}", payload);
    db::comments::get_comment(&con, &payload.id, payload.include_hidden).and_then(|p| {
        <Comment as TryInto<CommentPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

pub fn get_comments_in_thread(
    con: &DbConn,
    payload: GetCommentsPayload,
) -> IntResult<Vec<CommentPayload>> {
    trace!("get_comments_in_thread {:?}", payload);

    db::comments::get_comments_in_thread(&con, &payload.id, payload.include_hidden).and_then(
        |comments| {
            comments
                .into_iter()
                .map(|comment| comment.try_into())
                .collect::<Result<Vec<CommentPayload>, _>>()
                .context(IntErrorKind::ServerError)
                .map_err(|e| e.into())
        },
    )
}

pub fn get_all_comments(con: &DbConn, payload: GetHiddenPayload) -> IntResult<Vec<CommentPayload>> {
    trace!("get_all_comments {:?}", payload);

    db::comments::get_all_comments(&con, payload.include_hidden).and_then(|comments| {
        comments
            .into_iter()
            .map(|comment| comment.try_into())
            .collect::<Result<Vec<CommentPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

pub fn add_comment(con: &DbConn, payload: AddCommentPayload) -> IntResult<CommentPayload> {
    trace!("add_comment {:?}", payload);
    db::comments::insert_comment(
        &con,
        &payload.thread_id,
        &payload.user_id,
        &payload.parent_id,
        &payload.content,
    ).and_then(|p| {
        <Comment as TryInto<CommentPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

pub fn edit_comment(con: &DbConn, payload: EditCommentPayload) -> IntResult<CommentPayload> {
    trace!("edit_comment {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn hide_comment(con: &DbConn, payload: HideCommentPayload) -> IntResult<CommentPayload> {
    trace!("hide_comment {:?}", payload);
    db::comments::update_comment_hidden(&con, &payload.id, payload.hide).and_then(|p| {
        <Comment as TryInto<CommentPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

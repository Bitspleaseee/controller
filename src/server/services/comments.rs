use crate::db::{self, DbConn};
use crate::{IntResult, IntErrorKind};

use datatypes::content::requests::*;
use datatypes::content::responses::*;

pub fn get_comment(con: &DbConn, payload: GetCommentPayload) -> IntResult<CommentPayload> {
    trace!("get_comment {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn get_comments(con: &DbConn, payload: GetCommentsPayload) -> IntResult<Vec<CommentPayload>> {
    trace!("get_comments {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn get_all_comments( con: &DbConn, payload: GetHiddenPayload) -> IntResult<Vec<CommentPayload>> {
    trace!("get_all_comments {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn add_comment(con: &DbConn, payload: AddCommentPayload) -> IntResult<CommentPayload> {
    trace!("add_comment {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn edit_comment(con: &DbConn, payload: EditCommentPayload) -> IntResult<CommentPayload> {
    trace!("edit_comment {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn hide_comment(con: &DbConn, payload: HideCommentPayload) -> IntResult<CommentPayload> {
    trace!("hide_comment {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

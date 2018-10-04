use crate::db::{self, DbConn};

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::{ResponseError, ResponseResult};

pub fn get_comment(con: &DbConn, payload: GetCommentPayload) -> ResponseResult<CommentPayload> {
    trace!("get_comment {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn get_comments(
    con: &DbConn,
    payload: GetCommentsPayload,
) -> ResponseResult<Vec<CommentPayload>> {
    trace!("get_comments {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn add_comment(con: &DbConn, payload: AddCommentPayload) -> ResponseResult<CommentPayload> {
    trace!("add_comment {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn edit_comment(con: &DbConn, payload: EditCommentPayload) -> ResponseResult<CommentPayload> {
    trace!("edit_comment {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn hide_comment(con: &DbConn, payload: HideCommentPayload) -> ResponseResult<CommentPayload> {
    trace!("hide_comment {:?}", payload);
    Err(ResponseError::InternalServerError)
}

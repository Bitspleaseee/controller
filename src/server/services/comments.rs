use crate::db::{self, DbConn};

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseResult;

pub fn get_comment(con: &DbConn, payload: GetCommentPayload) -> ResponseResult<CommentPayload> {
    trace!("get_comment {:?}", payload);
    unimplemented!()
}

pub fn get_comments(con: &DbConn, payload: GetCommentsPayload) -> ResponseResult<Vec<CommentPayload>> {
    trace!("get_comments {:?}", payload);
    unimplemented!()
}

pub fn get_all_comments( con: &DbConn, payload: GetHiddenPayload) -> ResponseResult<Vec<CommentPayload>> {
    trace!("get_all_comments {:?}", payload);
    unimplemented!()
}

pub fn add_comment(con: &DbConn, payload: AddCommentPayload) -> ResponseResult<CommentPayload> {
    trace!("add_comment {:?}", payload);
    unimplemented!()
}

pub fn edit_comment(con: &DbConn, payload: EditCommentPayload) -> ResponseResult<CommentPayload> {
    trace!("edit_comment {:?}", payload);
    unimplemented!()
}

pub fn hide_comment(con: &DbConn, payload: HideCommentPayload) -> ResponseResult<CommentPayload> {
    trace!("hide_comment {:?}", payload);
    unimplemented!()
}

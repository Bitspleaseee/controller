use crate::db::{self, DbConn};

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::{ResponseError, ResponseResult};

pub fn get_thread(con: &DbConn, payload: GetThreadPayload) -> ResponseResult<ThreadPayload> {
    trace!("get_thread {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn get_threads(con: &DbConn, payload: GetThreadsPayload) -> ResponseResult<Vec<ThreadPayload>> {
    trace!("get_threads {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn add_thread(con: &DbConn, payload: AddThreadPayload) -> ResponseResult<ThreadPayload> {
    trace!("add_thread {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn edit_thread(con: &DbConn, payload: EditThreadPayload) -> ResponseResult<ThreadPayload> {
    trace!("edit_thread {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn hide_thread(con: &DbConn, payload: HideThreadPayload) -> ResponseResult<ThreadPayload> {
    trace!("hide_thread {:?}", payload);
    Err(ResponseError::InternalServerError)
}

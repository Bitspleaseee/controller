use crate::db::{self, DbConn};

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseResult;

pub fn get_thread(con: &DbConn, payload: GetThreadPayload) -> ResponseResult<ThreadPayload> {
    trace!("get_thread {:?}", payload);
    unimplemented!()
}

pub fn get_threads(con: &DbConn, payload: GetThreadsPayload) -> ResponseResult<Vec<ThreadPayload>> {
    trace!("get_threads {:?}", payload);
    unimplemented!()
}

pub fn get_all_threads(con: &DbConn, payload: GetHiddenPayload) -> ResponseResult<Vec<ThreadPayload>> {
    trace!("get_all_threads {:?}", payload);
    unimplemented!()
}

pub fn add_thread(con: &DbConn, payload: AddThreadPayload) -> ResponseResult<ThreadPayload> {
    trace!("add_thread {:?}", payload);
    unimplemented!()
}

pub fn edit_thread(con: &DbConn, payload: EditThreadPayload) -> ResponseResult<ThreadPayload> {
    trace!("edit_thread {:?}", payload);
    unimplemented!()
}

pub fn hide_thread(con: &DbConn, payload: HideThreadPayload) -> ResponseResult<ThreadPayload> {
    trace!("hide_thread {:?}", payload);
    unimplemented!()
}

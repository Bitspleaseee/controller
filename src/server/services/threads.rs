use crate::db::{self, DbConn};
use crate::{IntResult, IntErrorKind};

use datatypes::content::requests::*;
use datatypes::content::responses::*;

pub fn get_thread(con: &DbConn, payload: GetThreadPayload) -> IntResult<ThreadPayload> {
    trace!("get_thread {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn get_threads(con: &DbConn, payload: GetThreadsPayload) -> IntResult<Vec<ThreadPayload>> {
    trace!("get_threads {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn get_all_threads(con: &DbConn, payload: GetHiddenPayload) -> IntResult<Vec<ThreadPayload>> {
    trace!("get_all_threads {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn add_thread(con: &DbConn, payload: AddThreadPayload) -> IntResult<ThreadPayload> {
    trace!("add_thread {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn edit_thread(con: &DbConn, payload: EditThreadPayload) -> IntResult<ThreadPayload> {
    trace!("edit_thread {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn hide_thread(con: &DbConn, payload: HideThreadPayload) -> IntResult<ThreadPayload> {
    trace!("hide_thread {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

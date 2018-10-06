use failure::ResultExt;
use std::convert::TryInto;

use datatypes::content::requests::*;
use datatypes::content::responses::*;

use crate::db::{self, DbConn};
use crate::types::Thread;
use crate::{IntErrorKind, IntResult};

pub fn get_thread(con: &DbConn, payload: GetThreadPayload) -> IntResult<ThreadPayload> {
    trace!("get_thread {:?}", payload);
    db::threads::get_thread(&con, &payload.id, payload.include_hidden).and_then(|p| {
        <Thread as TryInto<ThreadPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

pub fn get_threads(con: &DbConn, payload: GetThreadsPayload) -> IntResult<Vec<ThreadPayload>> {
    trace!("get_threads {:?}", payload);

    db::threads::get_threads_in_category(&con, &payload.id, payload.include_hidden).and_then(
        |threads| {
            threads
                .into_iter()
                .map(|thread| thread.try_into())
                .collect::<Result<Vec<ThreadPayload>, _>>()
                .context(IntErrorKind::ServerError)
                .map_err(|e| e.into())
        },
    )
}

pub fn get_all_threads(con: &DbConn, payload: GetHiddenPayload) -> IntResult<Vec<ThreadPayload>> {
    trace!("get_all_threads {:?}", payload);

    db::threads::get_all_threads(&con, payload.include_hidden).and_then(|threads| {
        threads
            .into_iter()
            .map(|thread| thread.try_into())
            .collect::<Result<Vec<ThreadPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

pub fn add_thread(con: &DbConn, payload: AddThreadPayload) -> IntResult<ThreadPayload> {
    trace!("add_thread {:?}", payload);
    db::threads::insert_thread(&con, &payload.title, &payload.description).and_then(|p| {
        <Thread as TryInto<ThreadPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

pub fn edit_thread(con: &DbConn, payload: EditThreadPayload) -> IntResult<ThreadPayload> {
    trace!("edit_thread {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn hide_thread(con: &DbConn, payload: HideThreadPayload) -> IntResult<ThreadPayload> {
    trace!("hide_thread {:?}", payload);
    db::threads::update_thread_hidden(&con, &payload.id, payload.hide).and_then(|p| {
        <Thread as TryInto<ThreadPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

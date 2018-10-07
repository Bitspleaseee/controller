use failure::ResultExt;
use std::convert::TryInto;

use datatypes::content::requests::*;
use datatypes::content::responses::*;

use crate::db::{self, DbConn};
use crate::types::Thread;
use crate::{IntErrorKind, IntResult};

pub fn get_thread(con: &DbConn, payload: GetThreadPayload) -> IntResult<ThreadPayload> {
    let GetThreadPayload { id, include_hidden } = payload;
    trace!("get_thread: {:?}", payload);

    db::threads::get_thread(&con, id, include_hidden).and_then(|p| {
        <Thread as TryInto<ThreadPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert thread ({}) to payload: {}", id, e);
                e.into()
            })
    })
}

pub fn get_threads_in_category(
    con: &DbConn,
    payload: GetThreadsPayload,
) -> IntResult<Vec<ThreadPayload>> {
    let GetThreadsPayload { id, include_hidden } = payload;
    trace!("get_threads_in_category: {:?}", payload);

    db::threads::get_threads_in_category(&con, id, include_hidden).and_then(|threads| {
        threads
            .into_iter()
            .map(|thread| thread.try_into())
            .collect::<Result<Vec<ThreadPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert thread into payload: {}", e);
                e.into()
            })
    })
}

pub fn get_all_threads(con: &DbConn, payload: GetHiddenPayload) -> IntResult<Vec<ThreadPayload>> {
    let GetHiddenPayload { include_hidden } = payload;
    trace!("get_all_threads: {:?}", payload);

    db::threads::get_all_threads(&con, include_hidden).and_then(|threads| {
        threads
            .into_iter()
            .map(|thread| thread.try_into())
            .collect::<Result<Vec<ThreadPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert thread to payload: {}", e);
                e.into()
            })
    })
}

pub fn add_thread(con: &DbConn, payload: AddThreadPayload) -> IntResult<ThreadPayload> {
    trace!("add_thread: {:?}", payload);

    db::threads::insert_thread(&con, payload).and_then(|p| {
        <Thread as TryInto<ThreadPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert thread to payload: {}", e);
                e.into()
            })
    })
}

pub fn edit_thread(con: &DbConn, payload: EditThreadPayload) -> IntResult<ThreadPayload> {
    let EditThreadPayload { id, .. } = payload;

    trace!("edit_thread: {:?}", payload);

    db::threads::update_thread(&con, payload).and_then(|p| {
        <Thread as TryInto<ThreadPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert thread ({}) to payload: {}", id, e);
                e.into()
            })
    })
}

pub fn hide_thread(con: &DbConn, payload: HideThreadPayload) -> IntResult<ThreadPayload> {
    let HideThreadPayload { id, .. } = payload;

    trace!("hide_thread: {:?}", payload);

    db::threads::update_thread(&con, payload).and_then(|p| {
        <Thread as TryInto<ThreadPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert thread ({}) to payload: {}", id, e);
                e.into()
            })
    })
}

use crate::db::{self, DbConn};
use crate::types::User;
use crate::{IntErrorKind, IntResult};

use datatypes::content::requests::*;
use datatypes::content::responses::*;

use failure::ResultExt;
use std::convert::TryInto;

pub fn get_user(con: &DbConn, payload: GetUserPayload) -> IntResult<UserPayload> {
    let GetUserPayload { id } = payload;
    trace!("get_user: {:?}", payload);

    db::users::get_user(&con, id).and_then(|p| {
        trace!("got payload from db: {:?}", p);
        <User as TryInto<UserPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert user ({}) to payload: {}", id, e);
                e.into()
            })
    })
}

pub fn add_user(con: &DbConn, payload: AddUserPayload) -> IntResult<UserPayload> {
    trace!("add_user: {:?}", payload);

    db::users::insert_user(&con, payload).and_then(|p| {
        trace!("got payload from db: {:?}", p);
        <User as TryInto<UserPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert user to payload: {}", e);
                e.into()
            })
    })
}

pub fn edit_user(con: &DbConn, payload: EditUserPayload) -> IntResult<UserPayload> {
    trace!("edit_user {:?}", payload);

    db::users::update_user(con, payload).and_then(|p| {
        trace!("got payload from db: {:?}", p);
        <User as TryInto<UserPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert user to payload: {}", e);
                e.into()
            })
    })
}

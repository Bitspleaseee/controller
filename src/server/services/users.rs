use crate::db::{self, DbConn};
use crate::types::User;
use crate::{IntError, IntErrorKind, IntResult};

use datatypes::content::requests::*;
use datatypes::content::responses::*;

use failure::ResultExt;
use std::convert::TryInto;

pub fn get_user(con: &DbConn, payload: GetUserPayload) -> IntResult<UserPayload> {
    debug!("get_user: {:?}", payload);
    db::users::get_user(&con, &payload.id).and_then(|p| {
        debug!("got payload from db: {:?}", p);
        <User as TryInto<UserPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(IntError::from)
    })
}

pub fn add_user(con: &DbConn, payload: AddUserPayload) -> IntResult<UserPayload> {
    debug!("add_user: {:?}", payload);
    db::users::insert_user(&con, &payload.id, &payload.username).and_then(|p| {
        debug!("got payload from db: {:?}", p);
        <User as TryInto<UserPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(IntError::from)
    })
}

pub fn edit_user(con: &DbConn, payload: EditUserPayload) -> IntResult<UserPayload> {
    trace!("edit_user {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

pub fn upload_avatar(con: &DbConn, payload: UploadAvatarPayload) -> IntResult<UserPayload> {
    trace!("upload_avatar {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

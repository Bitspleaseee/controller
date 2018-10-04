use crate::db::{self, DbConn};
use crate::error::{Error, ErrorKind};
use crate::types::User;

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseResult;

use failure::ResultExt;
use std::convert::TryInto;

pub fn get_user(con: &DbConn, payload: GetUserPayload) -> ResponseResult<UserPayload> {
    trace!("get_user {:?}", payload);
    db::users::get_user(&con, &payload.id)
        .and_then(|p| {
            <User as TryInto<UserPayload>>::try_into(p)
                .context(ErrorKind::ServerError)
                .map_err(Error::from)
        }).map_err(|e| e.into())
}

pub fn add_user(con: &DbConn, payload: AddUserPayload) -> ResponseResult<UserPayload> {
    trace!("add_user {:?}", payload);
    db::users::insert_user(&con, &payload.id, &payload.username)
        .and_then(|p| {
            <User as TryInto<UserPayload>>::try_into(p)
                .context(ErrorKind::ServerError)
                .map_err(Error::from)
        }).map_err(|e| e.into())
}

pub fn edit_user(con: &DbConn, payload: EditUserPayload) -> ResponseResult<UserPayload> {
    trace!("edit_user {:?}", payload);
    unimplemented!()
}

pub fn upload_avatar(con: &DbConn, payload: UploadAvatarPayload) -> ResponseResult<UserPayload> {
    trace!("upload_avatar {:?}", payload);
    unimplemented!()
}

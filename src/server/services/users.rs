use crate::db::{self, DbConn};

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::{ResponseError, ResponseResult};

pub fn get_user(con: &DbConn, payload: GetUserPayload) -> ResponseResult<UserPayload> {
    trace!("get_user {:?}", payload);
    match db::users::get_user(&con, &payload.id) {
        Ok(value) => Ok(value.into()),
        Err(error) => Err(error.into()),
    }
}

pub fn add_user(con: &DbConn, payload: AddUserPayload) -> ResponseResult<UserPayload> {
    trace!("add_user {:?}", payload);
    match db::users::insert_user(&con, &payload.id, &payload.username) {
        Ok(value) => Ok(value.into()),
        Err(error) => Err(error.into()),
    }
}

pub fn edit_user(con: &DbConn, payload: EditUserPayload) -> ResponseResult<UserPayload> {
    trace!("edit_user {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn upload_avatar(con: &DbConn, payload: UploadAvatarPayload) -> ResponseResult<UserPayload> {
    trace!("upload_avatar {:?}", payload);
    Err(ResponseError::InternalServerError)
}

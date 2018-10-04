use crate::db::{self, DbConn};

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::{ResponseError, ResponseResult};

pub fn get_category(con: &DbConn, payload: GetCategoryPayload) -> ResponseResult<CategoryPayload> {
    trace!("get_category {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn get_categories(
    con: &DbConn,
    payload: GetCategoriesPayload,
) -> ResponseResult<Vec<CategoryPayload>> {
    trace!("get_categories {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn add_category(con: &DbConn, payload: AddCategoryPayload) -> ResponseResult<CategoryPayload> {
    trace!("add_category {:?}", payload);
    match db::categories::insert_category(&con, &payload.title, &payload.description) {
        Ok(value) => Ok(value.into()),
        Err(error) => Err(error.into()),
    }
}

pub fn edit_category(
    con: &DbConn,
    payload: EditCategoryPayload,
) -> ResponseResult<CategoryPayload> {
    trace!("edit_category {:?}", payload);
    Err(ResponseError::InternalServerError)
}

pub fn hide_category(
    con: &DbConn,
    payload: HideCategoryPayload,
) -> ResponseResult<CategoryPayload> {
    trace!("hide_category {:?}", payload);
    Err(ResponseError::InternalServerError)
}

use failure::ResultExt;
use std::convert::TryInto;

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseResult;

use crate::db::{self, DbConn};
use crate::error::ErrorKind;
use crate::types::Category;

pub fn get_category(con: &DbConn, payload: GetCategoryPayload) -> ResponseResult<CategoryPayload> {
    trace!("get_category {:?}", payload);
    unimplemented!()
}

pub fn get_categories(
    con: &DbConn,
    payload: GetCategoriesPayload,
) -> ResponseResult<Vec<CategoryPayload>> {
    trace!("get_categories {:?}", payload);
    unimplemented!()
}

pub fn add_category(con: &DbConn, payload: AddCategoryPayload) -> ResponseResult<CategoryPayload> {
    trace!("add_category {:?}", payload);
    db::categories::insert_category(&con, &payload.title, &payload.description)
        .and_then(|p| {
            <Category as TryInto<CategoryPayload>>::try_into(p)
                .context(ErrorKind::ServerError)
                .map_err(|e| e.into())
        }).map_err(|e| e.into())
}

pub fn edit_category(
    con: &DbConn,
    payload: EditCategoryPayload,
) -> ResponseResult<CategoryPayload> {
    trace!("edit_category {:?}", payload);
    unimplemented!()
}

pub fn hide_category(
    con: &DbConn,
    payload: HideCategoryPayload,
) -> ResponseResult<CategoryPayload> {
    trace!("hide_category {:?}", payload);
    unimplemented!()
}

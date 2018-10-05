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
    db::categories::get_category(&con, &payload.id)
        .and_then(|p| {
            <Category as TryInto<CategoryPayload>>::try_into(p)
                .context(ErrorKind::ServerError)
                .map_err(|e| e.into())
        }).map_err(|e| e.into())
}

pub fn get_categories(
    con: &DbConn,
    payload: GetHiddenPayload,
) -> ResponseResult<Vec<CategoryPayload>> {
    trace!("get_categories {:?}", payload);
    
    unimplemented!();
    
    // TODO
    
    /*
    // Convert from Iterator<Result<CategoryPayload, ValidationError>> to Vec<CategoryPayload>
    match db::categories::get_all_categories(&con, payload.include_hidden) {
        Ok(value) => Ok(value.into_iter().map(TryInto::try_into).collect()),
        Err(error) => Err(error.into()),
    }
    */

    /*
    // Cannot infer type for `B`
    db::categories::get_all_categories(&con, payload.include_hidden)
        .and_then(|p| {
            p.into_iter()
                .map(TryInto::<CategoryPayload>::try_into)
                .collect()
                .map_err(|e| e.into())
        }).map_err(|e| e.into())
    */
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
    // TODO: Implement edit_category
    // Make db::categories::update_category use a changeset and take a CategoryPayload
    // Requires From<CategoryPayload> for types::Category
}

pub fn hide_category(
    con: &DbConn,
    payload: HideCategoryPayload,
) -> ResponseResult<CategoryPayload> {
    trace!("hide_category {:?}", payload);
    db::categories::update_category_hidden(&con, &payload.id, payload.hide)
        .and_then(|p| {
            <Category as TryInto<CategoryPayload>>::try_into(p)
                .context(ErrorKind::ServerError)
                .map_err(|e| e.into())
        }).map_err(|e| e.into())
}

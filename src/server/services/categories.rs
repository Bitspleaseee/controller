use failure::ResultExt;
use std::convert::TryInto;

use datatypes::content::requests::*;
use datatypes::content::responses::*;

use crate::db::{self, DbConn};
use crate::types::Category;
use crate::{IntErrorKind, IntResult};

pub fn get_category(con: &DbConn, payload: GetCategoryPayload) -> IntResult<CategoryPayload> {
    trace!("get_category {:?}", payload);
    db::categories::get_category(&con, &payload.id, payload.include_hidden).and_then(|p| {
        <Category as TryInto<CategoryPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

pub fn get_all_categories(
    con: &DbConn,
    payload: GetHiddenPayload,
) -> IntResult<Vec<CategoryPayload>> {
    trace!("get_all_categories {:?}", payload);

    db::categories::get_all_categories(&con, payload.include_hidden).and_then(|categories| {
        categories
            .into_iter()
            .map(|category| category.try_into())
            .collect::<Result<Vec<CategoryPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

pub fn add_category(con: &DbConn, payload: AddCategoryPayload) -> IntResult<CategoryPayload> {
    trace!("add_category {:?}", payload);
    db::categories::insert_category(&con, &payload.title, &payload.description).and_then(|p| {
        <Category as TryInto<CategoryPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

pub fn edit_category(con: &DbConn, payload: EditCategoryPayload) -> IntResult<CategoryPayload> {
    trace!("edit_category {:?}", payload);

    Err(IntErrorKind::ServerError)?
    // TODO: Implement edit_category
    // Make db::categories::update_category use a changeset and take a CategoryPayload
    // Requires From<CategoryPayload> for types::Category
}

pub fn hide_category(con: &DbConn, payload: HideCategoryPayload) -> IntResult<CategoryPayload> {
    trace!("hide_category {:?}", payload);
    db::categories::update_category_hidden(&con, &payload.id, payload.hide).and_then(|p| {
        <Category as TryInto<CategoryPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| e.into())
    })
}

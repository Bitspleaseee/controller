use failure::ResultExt;
use std::convert::TryInto;

use datatypes::content::requests::*;
use datatypes::content::responses::*;

use crate::db::{self, DbConn};
use crate::types::Category;
use crate::{IntErrorKind, IntResult};

pub fn get_category(con: &DbConn, payload: GetCategoryPayload) -> IntResult<CategoryPayload> {
    let GetCategoryPayload { id, include_hidden } = payload;
    trace!("get_category: {:?}", payload);

    db::categories::get_category(&con, id, include_hidden).and_then(|p| {
        <Category as TryInto<CategoryPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert category to payload: {}", e);
                e.into()
            })
    })
}

pub fn get_all_categories(
    con: &DbConn,
    payload: GetHiddenPayload,
) -> IntResult<Vec<CategoryPayload>> {
    let GetHiddenPayload { include_hidden } = payload;
    trace!("get_all_categories: {:?}", payload);

    db::categories::get_all_categories(&con, include_hidden).and_then(|categories| {
        categories
            .into_iter()
            .map(|category| category.try_into())
            .collect::<Result<Vec<CategoryPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert category to payload: {}", e);
                e.into()
            })
    })
}

pub fn add_category(con: &DbConn, payload: AddCategoryPayload) -> IntResult<CategoryPayload> {
    trace!("add_category: {:?}", payload);

    db::categories::insert_category(&con, payload).and_then(|p| {
        <Category as TryInto<CategoryPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert category to payload: {}", e);
                e.into()
            })
    })
}

pub fn edit_category(con: &DbConn, payload: EditCategoryPayload) -> IntResult<CategoryPayload> {
    let id = payload.id;
    trace!("edit_category: {:?}", payload);

    db::categories::update_category(con, payload).and_then(|p| {
        <Category as TryInto<CategoryPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert category ({}) to payload: {}", id, e);
                e.into()
            })
    })
}

pub fn hide_category(con: &DbConn, payload: HideCategoryPayload) -> IntResult<CategoryPayload> {
    let id = payload.id;
    trace!("hide_category: {:?}", payload);

    db::categories::update_category(&con, payload).and_then(|p| {
        <Category as TryInto<CategoryPayload>>::try_into(p)
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert category ({}) to payload: {}", id, e);
                e.into()
            })
    })
}

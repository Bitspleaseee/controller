use diesel::prelude::*;
use failure::ResultExt;

use super::{DbConn, MAX_CATEGORY_LIMIT};
use crate::types::{Category, InsertCategory, UpdateCategory};
use crate::{IntErrorKind, IntResult};

use datatypes::valid::ids::*;

/// Inserts a new category into the category table
pub fn insert_category(con: &DbConn, category: impl Into<InsertCategory>) -> IntResult<Category> {
    use super::schema::categories::dsl;
    let category = category.into();

    trace!("Inserting category");

    category
        .insert_into(dsl::categories)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to insert category: {}", e);
            e.into()
        }).and_then(|_| {
            dsl::categories
                .order(dsl::id.desc())
                .first(con)
                .optional()
                .context(IntErrorKind::QueryError)?
                .ok_or(IntErrorKind::ContentNotFound)
                .map_err(|e| {
                    error!("Unable to get category after insertion: {}", e);
                    e.into()
                })
        })
}

/// Gets an exisiting category from the category table
pub fn get_category(con: &DbConn, id: CategoryId, include_hidden: bool) -> IntResult<Category> {
    use super::schema::categories::dsl;

    trace!("Getting category ({})", id);

    if include_hidden {
        dsl::categories
            .filter(dsl::id.eq(*id))
            .first::<Category>(con)
    } else {
        dsl::categories
            .filter(dsl::id.eq(*id))
            .filter(dsl::hidden.eq(false))
            .first::<Category>(con)
    }.optional()
    .context(IntErrorKind::QueryError)?
    .ok_or(IntErrorKind::ContentNotFound)
    .map_err(|e| {
        error!("Unable to get category ({}): {}", id, e);
        e.into()
    })
}

/// Gets all the categories from the category table
pub fn get_all_categories(con: &DbConn, include_hidden: bool) -> IntResult<Vec<Category>> {
    use super::schema::categories::dsl;

    trace!("Getting all categories, include hidden: {}", include_hidden);

    if include_hidden {
        dsl::categories.limit(MAX_CATEGORY_LIMIT).get_results(con)
    } else {
        dsl::categories
            .limit(MAX_CATEGORY_LIMIT)
            .filter(dsl::hidden.eq(false))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to get all categories: {}", e);
        e.into()
    })
}

/// Clears the category table
pub fn delete_all_categories(con: &DbConn) -> IntResult<usize> {
    use super::schema::categories::dsl;

    trace!("Deleting all categories");

    diesel::delete(dsl::categories)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to delete all categories: {}", e);
            e.into()
        })
}

/// Updates an existing category in the category table
pub fn update_category(con: &DbConn, category: impl Into<UpdateCategory>) -> IntResult<Category> {
    let category = category.into();
    let id = category.id;

    trace!("Updating category ({})", id);

    category
        .save_changes(con)
        .optional()
        .context(IntErrorKind::QueryError)?
        .ok_or(IntErrorKind::ContentNotFound)
        .map_err(|e| {
            error!("Unable to update category ({}): {}", id, e);
            e.into()
        })
}

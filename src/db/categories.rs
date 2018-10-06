use diesel::prelude::*;
use failure::ResultExt;

use super::DbConn;
use crate::types::Category;
use crate::{IntErrorKind, IntResult};

use datatypes::valid::fields::*;
use datatypes::valid::ids::*;

/// Inserts a new category into the category table
pub fn insert_category(
    connection: &DbConn,
    title: &Title,
    description: &Description,
) -> IntResult<Category> {
    use super::schema::categories::dsl;

    trace!("Inserting category: {}", title);

    diesel::insert_into(dsl::categories)
        .values((
            dsl::title.eq(title.as_ref()),
            dsl::description.eq(description.as_ref()),
        )).execute(connection)
        .context(IntErrorKind::QueryError)
        .and_then(|_| {
            dsl::categories
                .order(dsl::id.desc())
                .first(connection)
                .context(IntErrorKind::ContentNotFound)
        }).map_err(|e| e.into())
}

/// Gets an exisiting category from the category table
pub fn get_category(
    connection: &DbConn,
    id: &CategoryId,
    include_hidden: bool,
) -> IntResult<Category> {
    use super::schema::categories::dsl;

    trace!("Getting category ({:?})", id);

    if include_hidden {
        dsl::categories
            .filter(dsl::id.eq(*(*id)))
            .first::<Category>(connection)
    } else {
        dsl::categories
            .filter(dsl::id.eq(*(*id)))
            .filter(dsl::hidden.eq(false))
            .first::<Category>(connection)
    }.optional()
    .context(IntErrorKind::QueryError)?
    .ok_or(IntErrorKind::ContentNotFound)
    .map_err(|e| e.into())
}

/// Gets all the categories from the category table
pub fn get_all_categories(connection: &DbConn, include_hidden: bool) -> IntResult<Vec<Category>> {
    use super::schema::categories::dsl;

    trace!("Getting all categories, include hidden: {}", include_hidden);

    if include_hidden {
        dsl::categories.get_results(connection)
    } else {
        dsl::categories
            .filter(dsl::hidden.eq(false))
            .get_results(connection)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| e.into())
}

/// Clears the category table
pub fn delete_all_categories(connection: &DbConn) -> IntResult<usize> {
    use super::schema::categories::dsl;

    trace!("Deleting all categories");

    diesel::delete(dsl::categories)
        .execute(connection)
        .context(IntErrorKind::QueryError)
        .map_err(|e| e.into())
}

/// Updates an existing category in the category table
pub fn update_category(
    connection: &DbConn,
    id: &CategoryId,
    title: &Title,
    description: &Description,
) -> IntResult<Category> {
    use super::schema::categories::dsl;

    trace!("Updating category ({:?})", id);

    let num_updated = diesel::update(dsl::categories)
        .set((
            dsl::title.eq(title.as_ref()),
            dsl::description.eq(description.as_ref()),
        )).filter(dsl::id.eq(*(*id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_category(&connection, id, true)
    }
}

/// Updates the hidden flag for an existing category in the category table
pub fn update_category_hidden(
    connection: &DbConn,
    id: &CategoryId,
    hidden: bool,
) -> IntResult<Category> {
    use super::schema::categories::dsl;

    trace!("Updating category hidden flag ({:?})", id);

    let num_updated = diesel::update(dsl::categories)
        .set(dsl::hidden.eq(hidden))
        .filter(dsl::id.eq(*(*id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_category(&connection, id, true)
    }
}

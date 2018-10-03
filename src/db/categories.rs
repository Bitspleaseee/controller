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
    new_title: &Title,
    new_description: &Description,
) -> IntResult<Category> {
    use super::schema::categories::dsl::{categories, description, id, title};

    trace!("Inserting category: {}", new_title);

    diesel::insert_into(categories)
        .values((
            title.eq(new_title.as_ref()),
            description.eq(new_description.as_ref()),
        )).execute(connection)
        .context(IntErrorKind::QueryError)
        .and_then(|_| {
            categories
                .order(id.desc())
                .first(connection)
                .context(IntErrorKind::ContentNotFound)
        }).map_err(|e| e.into())
}

/// Gets an exisiting category from the category table
fn get_category(connection: &DbConn, category_id: &CategoryId) -> IntResult<Category> {
    use super::schema::categories::dsl::{categories, id};

    trace!("Getting category ({:?})", category_id);

    categories
        .filter(id.eq(*(*category_id)))
        .first::<Category>(connection)
        .optional()
        .context(IntErrorKind::QueryError)?
        .ok_or(IntErrorKind::ContentNotFound)
        .map_err(|e| e.into())
}

/// Gets all the categories from the category table
pub fn get_all_categories(connection: &DbConn, include_hidden: bool) -> IntResult<Vec<Category>> {
    use super::schema::categories::dsl::{categories, hidden};

    trace!("Getting all categories, include hidden: {}", include_hidden);

    if include_hidden {
        categories.get_results(connection)
    } else {
        categories.filter(hidden.eq(false)).get_results(connection)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| e.into())
}

/// Clears the category table
pub fn delete_all_categories(connection: &DbConn) -> IntResult<usize> {
    use super::schema::categories::dsl::categories;

    trace!("Deleting all categories");

    diesel::delete(categories)
        .execute(connection)
        .context(IntErrorKind::QueryError)
        .map_err(|e| e.into())
}

/// Updates an existing category in the category table
pub fn update_category(
    connection: &DbConn,
    category_id: &CategoryId,
    new_title: &Title,
    new_description: &Description,
) -> IntResult<Category> {
    use super::schema::categories::dsl::{categories, description, id, title};

    trace!("Updating category ({:?})", category_id);

    let num_updated = diesel::update(categories)
        .set((
            title.eq(new_title.as_ref()),
            description.eq(new_description.as_ref()),
        )).filter(id.eq(*(*category_id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        //get_category(&connection, )
        Err(IntErrorKind::ContentNotFound)?
    }
}

/// Updates the title for an existing category in the category table
pub fn update_category_title(
    connection: &DbConn,
    category_id: &CategoryId,
    new_title: &Title,
) -> IntResult<Category> {
    use super::schema::categories::dsl::{categories, id, title};

    trace!("Updating category title ({:?})", category_id);

    let num_updated = diesel::update(categories)
        .set(title.eq(new_title.as_ref()))
        .filter(id.eq(*(*category_id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_category(&connection, category_id)
    }
}

/// Updates the description for an existing category in the category table
pub fn update_category_description(
    connection: &DbConn,
    category_id: &CategoryId,
    new_description: &Description,
) -> IntResult<Category> {
    use super::schema::categories::dsl::{categories, description, id};

    trace!("Updating category description ({:?})", category_id);

    let num_updated = diesel::update(categories)
        .set(description.eq(new_description.as_ref()))
        .filter(id.eq(*(*category_id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_category(&connection, category_id)
    }
}

/// Updates the hidden flag for an existing category in the category table
pub fn update_category_hidden(
    connection: &DbConn,
    category_id: &CategoryId,
    new_hidden: &bool,
) -> IntResult<Category> {
    use super::schema::categories::dsl::{categories, hidden, id};

    trace!("Updating category hidden flag ({:?})", category_id);

    let num_updated = diesel::update(categories)
        .set(hidden.eq(new_hidden))
        .filter(id.eq(*(*category_id)))
        .execute(connection)
        .context(IntErrorKind::QueryError)?;

    if num_updated == 0 {
        Err(IntErrorKind::ContentNotFound)?
    } else {
        get_category(&connection, category_id)
    }
}

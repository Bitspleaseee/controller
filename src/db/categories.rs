use diesel::prelude::*;
use log::*;

use super::super::types::Category;
use super::{DbConn, Error};

/// Inserts a new category into the category table
pub fn insert_category(
    connection: &DbConn,
    new_title: &str,
    new_description: &str,
) -> Result<Category, Error> {
    use super::schema::categories::dsl::{categories, description, id, title};

    trace!("Inserting category: {}", new_title);

    let result = diesel::insert_into(categories)
        .values((title.eq(new_title), description.eq(new_description)))
        .execute(connection);

    match result {
        Ok(_) => match categories.order(id.desc()).first(connection).ok() {
            Some(user) => Ok(user),
            None => Err(Error::NotFound),
        },
        Err(error) => {
            error!("Error inserting category ({}): {}", new_title, error);
            Err(Error::Database)
        }
    }
}

/// Gets an exisiting category from the category table
fn get_category_con(connection: &DbConn, category_id: i32) -> Result<Category, Error> {
    use super::schema::categories::dsl::{categories, id};

    trace!("Getting category ({})", category_id);

    let result = categories
        .filter(id.eq(category_id))
        .first::<Category>(connection)
        .optional();

    match result {
        Err(error) => {
            error!("Error getting category ({}): {}", category_id, error);
            Err(Error::Database)
        }
        Ok(row) => match row {
            None => Err(Error::NotFound),
            Some(user) => Ok(user),
        },
    }
}

/// Gets all the categories from the category table
pub fn get_all_categories(
    connection: &DbConn,
    include_hidden: bool,
) -> Result<Vec<Category>, Error> {
    use super::schema::categories::dsl::{categories, hidden};

    trace!("Getting all categories, include hidden: {}", include_hidden);

    let result = if include_hidden {
        categories.get_results(connection)
    } else {
        categories.filter(hidden.eq(false)).get_results(connection)
    };

    match result {
        Ok(rows) => Ok(rows),
        Err(error) => {
            error!("Error getting all categories: {}", error);
            Err(Error::Database)
        }
    }
}

/// Clears the category table
pub fn delete_all_categories(connection: &DbConn) -> Result<usize, Error> {
    use super::schema::categories::dsl::categories;

    trace!("Deleting all categories");

    let result = diesel::delete(categories).execute(connection);

    match result {
        Ok(num_deleted) => Ok(num_deleted),
        Err(error) => {
            error!("Error deleting all categories: {}", error);
            Err(Error::Database)
        }
    }
}

/// Gets the updated category or an error based on the result of the update statement
fn get_update_result(
    result: Result<usize, diesel::result::Error>,
    connection: &DbConn,
    category_id: i32,
) -> Result<Category, Error> {
    match result {
        Ok(num_updated) => {
            if num_updated == 0 {
                Err(Error::NotFound)
            } else {
                get_category_con(&connection, category_id)
            }
        }
        Err(error) => {
            error!("Error updating category ({}): {}", category_id, error);
            Err(Error::Database)
        }
    }
}

/// Updates an existing category in the category table
pub fn update_category(connection: &DbConn, category: &Category) -> Result<Category, Error> {
    use super::schema::categories::dsl::{categories, id};

    trace!("Updating category ({}:{})", category.id, category.title);

    let result = diesel::update(categories)
        .set(category)
        .filter(id.eq(category.id))
        .execute(connection);

    get_update_result(result, connection, category.id)
}

/// Updates the title for an existing category in the category table
pub fn update_category_title(
    connection: &DbConn,
    category_id: i32,
    new_title: &str,
) -> Result<Category, Error> {
    use super::schema::categories::dsl::{categories, id, title};

    trace!("Updating category title ({})", category_id);

    let result = diesel::update(categories)
        .set(title.eq(new_title))
        .filter(id.eq(category_id))
        .execute(connection);

    get_update_result(result, connection, category_id)
}

/// Updates the description for an existing category in the category table
pub fn update_category_description(
    connection: &DbConn,
    category_id: i32,
    new_description: &str,
) -> Result<Category, Error> {
    use super::schema::categories::dsl::{categories, description, id};

    trace!("Updating category description ({})", category_id);

    let result = diesel::update(categories)
        .set(description.eq(new_description))
        .filter(id.eq(category_id))
        .execute(connection);

    get_update_result(result, connection, category_id)
}

/// Updates the hidden flag for an existing category in the category table
pub fn update_category_hidden(
    connection: &DbConn,
    category_id: i32,
    new_hidden: bool,
) -> Result<Category, Error> {
    use super::schema::categories::dsl::{categories, hidden, id};

    trace!("Updating category hidden flag ({})", category_id);

    let result = diesel::update(categories)
        .set(hidden.eq(new_hidden))
        .filter(id.eq(category_id))
        .execute(connection);

    get_update_result(result, connection, category_id)
}

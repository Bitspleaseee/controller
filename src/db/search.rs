use diesel::prelude::*;
use failure::ResultExt;

use super::{DbConn, MAX_SEARCH_LIMIT};
use crate::types::{Category, Comment, SearchResults, Thread, User};
use crate::{IntErrorKind, IntResult};

use datatypes::content::requests::*;

pub fn search_user(con: &DbConn, query: &str) -> IntResult<Vec<User>> {
    use super::schema::users::dsl;
    dsl::users
        .limit(MAX_SEARCH_LIMIT)
        .filter(dsl::username.like(query))
        .get_results(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to search users: {}", e);
            e.into()
        })
}

pub fn search_category(
    con: &DbConn,
    query: &str,
    include_hidden: bool,
) -> IntResult<Vec<Category>> {
    use super::schema::categories::dsl;
    if include_hidden {
        dsl::categories
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::title.like(query))
            .get_results(con)
    } else {
        dsl::categories
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::hidden.eq(false))
            .filter(dsl::title.like(query))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to search categories: {}", e);
        e.into()
    })
}

pub fn search_thread(con: &DbConn, query: &str, include_hidden: bool) -> IntResult<Vec<Thread>> {
    use super::schema::threads::dsl;
    if include_hidden {
        dsl::threads
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::title.like(query))
            .or_filter(dsl::description.like(query))
            .get_results(con)
    } else {
        dsl::threads
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::hidden.eq(false))
            .filter(dsl::title.like(query).or(dsl::description.like(query)))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to search theads: {}", e);
        e.into()
    })
}

pub fn search_comment(con: &DbConn, query: &str, include_hidden: bool) -> IntResult<Vec<Comment>> {
    use super::schema::comments::dsl;
    if include_hidden {
        dsl::comments
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::content.like(query))
            .get_results(con)
    } else {
        dsl::comments
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::hidden.eq(false))
            .filter(dsl::content.like(query))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to search comments: {}", e);
        e.into()
    })
}

/// Gets all the categories from the category table
pub fn search(con: &DbConn, payload: SearchPayload) -> IntResult<SearchResults> {
    trace!("Searching, include hidden: {}", payload.include_hidden);

    let query = "%".to_owned() + payload.query.as_ref() + "%";

    let users = search_user(con, &query)?;
    let categories = search_category(con, &query, payload.include_hidden)?;
    let threads = search_thread(con, &query, payload.include_hidden)?;
    let comments = search_comment(con, &query, payload.include_hidden)?;

    Ok(SearchResults {
        categories,
        threads,
        comments,
        users,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{categories, comments, establish_connection, threads, users};
    use crate::types::{InsertCategory, InsertComment, InsertThread, InsertUser};
    use std::convert::TryInto;

    #[test]
    fn search() {
        let con = establish_connection(&std::env::var("CONTROLLER_DATABASE_URL").unwrap()).unwrap();
        assert!(comments::delete_all_comments(&con).is_ok());
        assert!(threads::delete_all_threads(&con).is_ok());
        assert!(categories::delete_all_categories(&con).is_ok());
        assert!(users::delete_all_users(&con).is_ok());
        add_data(&con);

        let payload = SearchPayload {
            query: "aaa".to_string().try_into().unwrap(),
            include_hidden: true,
        };
        println!("{:#?}", payload);

        let res = super::search(&con, payload);
        assert!(res.is_ok());
        let res = res.unwrap();
        println!("{:#?}", res);
    }

    fn add_data(con: &DbConn) {
        // Users
        let insert_data = InsertUser {
            id: 30,
            username: "foofoo".to_string(),
        };
        let returned_data = users::insert_user(&con, insert_data);
        assert!(returned_data.is_ok());
        let user_foofoo = returned_data.unwrap();

        let insert_data = InsertUser {
            id: 31,
            username: "barbar".to_string(),
        };
        let returned_data = users::insert_user(&con, insert_data);
        assert!(returned_data.is_ok());
        let user_barbar = returned_data.unwrap();

        // Categories
        let insert_data = InsertCategory {
            title: "aaaaaaaaaaaa".to_string(),
            description: "bbbbbbbbbb".to_string(),
        };
        let returned_data = categories::insert_category(&con, insert_data);
        assert!(returned_data.is_ok());
        let category_ab = returned_data.unwrap();

        let insert_data = InsertCategory {
            title: "cccccccccc".to_string(),
            description: "dddddddddd".to_string(),
        };
        let returned_data = categories::insert_category(&con, insert_data);
        assert!(returned_data.is_ok());
        let category_cd = returned_data.unwrap();

        // Threads
        let insert_data = InsertThread {
            category_id: category_ab.id,
            user_id: user_foofoo.id,
            title: "eeeeeeeeee".to_string(),
            description: "ffffffffff".to_string(),
        };
        let returned_data = threads::insert_thread(&con, insert_data);
        assert!(returned_data.is_ok());
        let thread_ef = returned_data.unwrap();

        let insert_data = InsertThread {
            category_id: category_cd.id,
            user_id: user_barbar.id,
            title: "gggggggggg".to_string(),
            description: "hhhhhhhhhh".to_string(),
        };
        let returned_data = threads::insert_thread(&con, insert_data);
        assert!(returned_data.is_ok());
        let thread_gh = returned_data.unwrap();

        // Comments
        let insert_data = InsertComment {
            thread_id: thread_ef.id,
            user_id: user_barbar.id,
            parent_id: None,
            content: "iiiiiiiiii".to_string(),
        };
        assert!(comments::insert_comment(&con, insert_data).is_ok());

        let insert_data = InsertComment {
            thread_id: thread_gh.id,
            user_id: user_foofoo.id,
            parent_id: None,
            content: "jjjjjjjjjj".to_string(),
        };
        assert!(comments::insert_comment(&con, insert_data).is_ok());
    }
}

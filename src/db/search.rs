use diesel::prelude::*;
use failure::ResultExt;
use std::convert::TryInto;

use super::{DbConn, MAX_SEARCH_LIMIT};
use crate::types::{Category, Comment, Thread, User};
use crate::{IntErrorKind, IntResult};

use datatypes::content::requests::*;
use datatypes::content::responses::*;

pub fn search_user(con: &DbConn, payload: &SearchPayload) -> IntResult<Vec<User>> {
    use super::schema::users::dsl;
    dsl::users
        .limit(MAX_SEARCH_LIMIT)
        .filter(dsl::username.like(payload.query.as_ref()))
        .get_results(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to search users: {}", e);
            e.into()
        })
}

pub fn search_category(con: &DbConn, payload: &SearchPayload) -> IntResult<Vec<Category>> {
    use super::schema::categories::dsl;
    if payload.include_hidden {
        dsl::categories
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::title.like(payload.query.as_ref()))
            .get_results(con)
    } else {
        dsl::categories
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::hidden.eq(false))
            .filter(dsl::title.like(payload.query.as_ref()))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to search categories: {}", e);
        e.into()
    })
}

pub fn search_thread(con: &DbConn, payload: &SearchPayload) -> IntResult<Vec<Thread>> {
    use super::schema::threads::dsl;
    if payload.include_hidden {
        dsl::threads
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::title.like(payload.query.as_ref()))
            .or_filter(dsl::description.like(payload.query.as_ref()))
            .get_results(con)
    } else {
        dsl::threads
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::hidden.eq(false))
            .filter(dsl::title.like(payload.query.as_ref()))
            .or_filter(dsl::description.like(payload.query.as_ref()))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to search theads: {}", e);
        e.into()
    })
}

pub fn search_comment(con: &DbConn, payload: &SearchPayload) -> IntResult<Vec<Comment>> {
    use super::schema::comments::dsl;
    if payload.include_hidden {
        dsl::comments
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::content.like(payload.query.as_ref()))
            .get_results(con)
    } else {
        dsl::comments
            .limit(MAX_SEARCH_LIMIT)
            .filter(dsl::hidden.eq(false))
            .filter(dsl::content.like(payload.query.as_ref()))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to search comments: {}", e);
        e.into()
    })
}

/// Gets all the categories from the category table
pub fn search(con: &DbConn, payload: SearchPayload) -> IntResult<SearchResultsPayload> {
    trace!("Searching, include hidden: {}", payload.include_hidden);

    let users = search_user(con, &payload).and_then(|list| {
        list.into_iter()
            .map(|i| i.try_into())
            .collect::<Result<Vec<UserPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert user to payload: {}", e);
                e.into()
            })
    })?;

    let categories = search_category(con, &payload).and_then(|list| {
        list.into_iter()
            .map(|i| i.try_into())
            .collect::<Result<Vec<CategoryPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert category to payload: {}", e);
                e.into()
            })
    })?;

    let threads = search_thread(con, &payload).and_then(|list| {
        list.into_iter()
            .map(|i| i.try_into())
            .collect::<Result<Vec<ThreadPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert thread to payload: {}", e);
                e.into()
            })
    })?;

    let comments = search_comment(con, &payload).and_then(|list| {
        list.into_iter()
            .map(|i| i.try_into())
            .collect::<Result<Vec<CommentPayload>, _>>()
            .context(IntErrorKind::ServerError)
            .map_err(|e| {
                error!("Unable to convert comment to payload: {}", e);
                e.into()
            })
    })?;

    Ok(SearchResultsPayload {
        categories,
        threads,
        comments,
        users,
    })
}

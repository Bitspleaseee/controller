use crate::db::{self, DbConn};
use crate::{IntErrorKind, IntResult};

use failure::ResultExt;
use std::convert::TryInto;

use datatypes::content::requests::*;
use datatypes::content::responses::*;

pub fn search(con: &DbConn, payload: SearchPayload) -> IntResult<SearchResultsPayload> {
    trace!("search {:?}", payload);

    let res = db::search::search(con, payload)?;

    let users = res
        .users
        .into_iter()
        .map(|i| i.try_into())
        .collect::<Result<Vec<UserPayload>, _>>()
        .context(IntErrorKind::ServerError)?;

    let categories = res
        .categories
        .into_iter()
        .map(|i| i.try_into())
        .collect::<Result<Vec<CategoryPayload>, _>>()
        .context(IntErrorKind::ServerError)?;

    let threads = res
        .threads
        .into_iter()
        .map(|i| i.try_into())
        .collect::<Result<Vec<ThreadPayload>, _>>()
        .context(IntErrorKind::ServerError)?;

    let comments = res
        .comments
        .into_iter()
        .map(|i| i.try_into())
        .collect::<Result<Vec<CommentPayload>, _>>()
        .context(IntErrorKind::ServerError)?;

    Ok(SearchResultsPayload {
        categories,
        threads,
        comments,
        users,
    })
}

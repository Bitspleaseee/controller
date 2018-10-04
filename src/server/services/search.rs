use crate::db::{self, DbConn};

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::{ResponseError, ResponseResult};

pub fn search(con: &DbConn, payload: SearchPayload) -> ResponseResult<SearchResultsPayload> {
    trace!("search {:?}", payload);
    Err(ResponseError::InternalServerError)
}

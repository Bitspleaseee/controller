use crate::db::{self, DbConn};
use crate::{IntResult, IntErrorKind};

use datatypes::content::requests::*;
use datatypes::content::responses::*;

pub fn search(con: &DbConn, payload: SearchPayload) -> IntResult<SearchResultsPayload> {
    trace!("search {:?}", payload);
    Err(IntErrorKind::ServerError)?
}

use diesel::prelude::*;
use failure::ResultExt;

use super::{DbConn, MAX_COMMENT_LIMIT};
use crate::types::{Comment, InsertComment, UpdateComment};
use crate::{IntErrorKind, IntResult};

use datatypes::valid::ids::*;

/// Inserts a new comment into the comment table
pub fn insert_comment(con: &DbConn, comment: impl Into<InsertComment>) -> IntResult<Comment> {
    use super::schema::comments::dsl;
    let comment = comment.into();
    let user_id = comment.user_id;

    trace!("Inserting comment");

    comment
        .insert_into(dsl::comments)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to insert comment: {}", e);
            e.into()
        }).and_then(|_| {
            dsl::comments
                .filter(dsl::user_id.eq(user_id))
                .order(dsl::id.desc())
                .first(con)
                .optional()
                .context(IntErrorKind::QueryError)?
                .ok_or(IntErrorKind::ContentNotFound)
                .map_err(|e| {
                    error!("Unable to get comment after insertion: {}", e);
                    e.into()
                })
        })
}

/// Gets an exisiting comment from the comment table
pub fn get_comment(con: &DbConn, id: CommentId, include_hidden: bool) -> IntResult<Comment> {
    use super::schema::comments::dsl;

    trace!("Getting comment ({})", id);

    if include_hidden {
        dsl::comments.filter(dsl::id.eq(*id)).first::<Comment>(con)
    } else {
        dsl::comments
            .filter(dsl::id.eq(*id))
            .filter(dsl::hidden.eq(false))
            .first::<Comment>(con)
    }.optional()
    .context(IntErrorKind::QueryError)?
    .ok_or(IntErrorKind::ContentNotFound)
    .map_err(|e| {
        error!("Unable to get comment ({}): {}", id, e);
        e.into()
    })
}

/// Gets all the comments from the comment table
pub fn get_all_comments(con: &DbConn, include_hidden: bool) -> IntResult<Vec<Comment>> {
    use super::schema::comments::dsl;

    trace!("Getting all comments, include hidden: {}", include_hidden);

    if include_hidden {
        dsl::comments.limit(MAX_COMMENT_LIMIT).get_results(con)
    } else {
        dsl::comments
            .limit(MAX_COMMENT_LIMIT)
            .filter(dsl::hidden.eq(false))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to get comments: {}", e);
        e.into()
    })
}

/// Gets all the comments in a thread from the comment table
pub fn get_comments_in_thread(
    con: &DbConn,
    id: ThreadId,
    include_hidden: bool,
) -> IntResult<Vec<Comment>> {
    use super::schema::comments::dsl;

    trace!("Getting comments in thread ({})", id);

    if include_hidden {
        dsl::comments
            .limit(MAX_COMMENT_LIMIT)
            .filter(dsl::thread_id.eq(*id))
            .get_results(con)
    } else {
        dsl::comments
            .limit(MAX_COMMENT_LIMIT)
            .filter(dsl::thread_id.eq(*id))
            .filter(dsl::hidden.eq(false))
            .get_results(con)
    }.context(IntErrorKind::QueryError)
    .map_err(|e| {
        error!("Unable to get comments in thread ({}): {}", id, e);
        e.into()
    })
}

/// Clears the comment table
pub fn delete_all_comments(con: &DbConn) -> IntResult<usize> {
    use super::schema::comments::dsl;

    trace!("Deleting all comments");

    diesel::delete(dsl::comments)
        .execute(con)
        .context(IntErrorKind::QueryError)
        .map_err(|e| {
            error!("Unable to delete all comments: {}", e);
            e.into()
        })
}

/// Updates an existing comment in the comment table
pub fn update_comment(
    con: &DbConn,
    user_id: UserId,
    comment: impl Into<UpdateComment>,
) -> IntResult<Comment> {
    use super::schema::comments::dsl;

    let comment = comment.into();
    let id = comment.id;

    trace!("Updating comment ({})", id);

    let updated = diesel::update(dsl::comments)
        .filter(dsl::id.eq(id))
        .filter(dsl::user_id.eq(*user_id))
        .set(&comment)
        .execute(con)
        .context(IntErrorKind::QueryError)?;

    if updated == 0 {
        return Err(IntErrorKind::ContentNotFound.into());
    }

    get_comment(con, id.into(), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use crate::db::{categories, establish_connection, threads, users};
    use crate::types::{InsertCategory, InsertThread, InsertUser};

    #[test]
    fn insert_and_get() {
        let con = establish_connection(&std::env::var("CONTROLLER_DATABASE_URL").unwrap()).unwrap();

        // User
        let insert_data = InsertUser {
            id: 20,
            username: "TestUser".to_string(),
        };
        let returned_data = users::insert_user(&con, insert_data);
        assert!(returned_data.is_ok());
        let user = returned_data.unwrap();

        // Category
        let insert_data = InsertCategory {
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };
        let returned_data = categories::insert_category(&con, insert_data);
        assert!(returned_data.is_ok());
        let category = returned_data.unwrap();

        // Thread
        let insert_data = InsertThread {
            category_id: category.id,
            user_id: user.id,
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };
        let returned_data = threads::insert_thread(&con, insert_data);
        assert!(returned_data.is_ok());
        let thread = returned_data.unwrap();

        // Comment
        let insert_data = InsertComment {
            thread_id: thread.id,
            user_id: user.id,
            parent_id: None,
            content: "TestContent".to_string(),
        };

        let insert_data_pid = InsertComment {
            thread_id: thread.id,
            user_id: user.id,
            parent_id: Some(0),
            content: "TestContent".to_string(),
        };

        let insert_data_uid = InsertComment {
            thread_id: thread.id,
            user_id: 0,
            parent_id: None,
            content: "TestContent".to_string(),
        };

        let insert_data_tid = InsertComment {
            thread_id: 0,
            user_id: user.id,
            parent_id: None,
            content: "TestContent".to_string(),
        };

        let mut expected_data = Comment {
            id: 1,
            thread_id: thread.id,
            parent_id: None,
            user_id: user.id,
            content: "TestContent".to_string(),
            timestamp: NaiveDateTime::from_timestamp(0, 0),
            hidden: false,
        };

        // Missing foreign keys
        assert!(insert_comment(&con, insert_data_pid).is_err());
        assert!(insert_comment(&con, insert_data_uid).is_err());
        assert!(insert_comment(&con, insert_data_tid).is_err());

        // Insert
        let returned_data = insert_comment(&con, insert_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Compare
        expected_data.id = returned_data.id;
        expected_data.timestamp = returned_data.timestamp;
        assert_eq!(returned_data, expected_data);

        // Get
        let returned_data = get_comment(&con, returned_data.id.into(), false);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Compare
        assert_eq!(returned_data, expected_data);
    }

    #[test]
    fn update() {
        let con = establish_connection(&std::env::var("CONTROLLER_DATABASE_URL").unwrap()).unwrap();

        // User
        let insert_data = InsertUser {
            id: 21,
            username: "TestUser".to_string(),
        };
        let returned_data = users::insert_user(&con, insert_data);
        assert!(returned_data.is_ok());
        let user = returned_data.unwrap();

        // Category
        let insert_data = InsertCategory {
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };
        let returned_data = categories::insert_category(&con, insert_data);
        assert!(returned_data.is_ok());
        let category = returned_data.unwrap();

        // Thread
        let insert_data = InsertThread {
            category_id: category.id,
            user_id: user.id,
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };
        let returned_data = threads::insert_thread(&con, insert_data);
        assert!(returned_data.is_ok());
        let thread = returned_data.unwrap();

        // Comment
        let insert_data = InsertComment {
            thread_id: thread.id,
            user_id: user.id,
            parent_id: None,
            content: "TestContent".to_string(),
        };

        let mut update_data = UpdateComment {
            id: 1,
            content: Some("OtherContent".to_string()),
            hidden: Some(true),
        };

        let mut expected_data = Comment {
            id: 1,
            thread_id: thread.id,
            parent_id: None,
            user_id: user.id,
            content: "OtherContent".to_string(),
            timestamp: NaiveDateTime::from_timestamp(0, 0),
            hidden: true,
        };

        // Insert
        let returned_data = insert_comment(&con, insert_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Update
        update_data.id = returned_data.id;
        let returned_data = update_comment(&con, user.id.into(), update_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Compare
        expected_data.id = returned_data.id;
        expected_data.timestamp = returned_data.timestamp;
        assert_eq!(returned_data, expected_data);
    }

    #[test]
    fn hide() {
        let con = establish_connection(&std::env::var("CONTROLLER_DATABASE_URL").unwrap()).unwrap();

        // User
        let insert_data = InsertUser {
            id: 32,
            username: "TestUser".to_string(),
        };
        let returned_data = users::insert_user(&con, insert_data);
        assert!(returned_data.is_ok());
        let user = returned_data.unwrap();

        // Category
        let insert_data = InsertCategory {
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };
        let returned_data = categories::insert_category(&con, insert_data);
        assert!(returned_data.is_ok());
        let category = returned_data.unwrap();

        // Thread
        let insert_data = InsertThread {
            category_id: category.id,
            user_id: user.id,
            title: "TestTitle".to_string(),
            description: "TestDescription".to_string(),
        };
        let returned_data = threads::insert_thread(&con, insert_data);
        assert!(returned_data.is_ok());
        let thread = returned_data.unwrap();

        // Comment
        let insert_data = InsertComment {
            thread_id: thread.id,
            user_id: user.id,
            parent_id: None,
            content: "TestContent".to_string(),
        };

        let mut update_data = UpdateComment {
            id: 1,
            content: None,
            hidden: Some(true),
        };

        // insert
        let returned_data = insert_comment(&con, insert_data);
        assert!(returned_data.is_ok());
        let returned_data = returned_data.unwrap();

        // Get
        assert!(get_comment(&con, returned_data.id.into(), false).is_ok());

        // Delete
        update_data.id = returned_data.id;
        assert!(update_comment(&con, user.id.into(), update_data).is_ok());

        // Fail to get
        assert!(get_comment(&con, returned_data.id.into(), false).is_err());
    }
}

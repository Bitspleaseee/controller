use super::Server;

use futures_cpupool::CpuFuture;

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseError;

mod categories;
mod comments;
mod search;
mod threads;
mod users;

service! {
    rpc get_user(payload: GetUserPayload) -> UserPayload | ResponseError;
    rpc add_user(payload: AddUserPayload) -> UserPayload | ResponseError;
    rpc edit_user(payload: EditUserPayload) -> UserPayload | ResponseError;
    rpc upload_avatar(payload: UploadAvatarPayload) -> UserPayload | ResponseError;

    rpc get_category(payload: GetCategoryPayload) -> CategoryPayload | ResponseError;
    rpc get_categories(payload: GetHiddenPayload) -> Vec<CategoryPayload> | ResponseError;
    rpc add_category(payload: AddCategoryPayload) -> CategoryPayload | ResponseError;
    rpc edit_category(payload: EditCategoryPayload) -> CategoryPayload | ResponseError;
    rpc hide_category(payload: HideCategoryPayload) -> CategoryPayload | ResponseError;

    rpc get_thread(payload: GetThreadPayload) -> ThreadPayload | ResponseError;
    rpc get_threads(payload: GetThreadsPayload) -> Vec<ThreadPayload> | ResponseError;
    rpc get_all_threads(payload: GetHiddenPayload) -> Vec<ThreadPayload> | ResponseError;
    rpc add_thread(payload: AddThreadPayload) -> ThreadPayload | ResponseError;
    rpc edit_thread(payload: EditThreadPayload) -> ThreadPayload | ResponseError;
    rpc hide_thread(payload: HideThreadPayload) -> ThreadPayload | ResponseError;

    rpc get_comment(payload: GetCommentPayload) -> CommentPayload | ResponseError;
    rpc get_comments(payload: GetCommentsPayload) -> Vec<CommentPayload> | ResponseError;
    rpc get_all_comments(payload: GetHiddenPayload) -> Vec<CommentPayload> | ResponseError;
    rpc add_comment(payload: AddCommentPayload) -> CommentPayload | ResponseError;
    rpc edit_comment(payload: EditCommentPayload) -> CommentPayload | ResponseError;
    rpc hide_comment(payload: HideCommentPayload) -> CommentPayload | ResponseError;

    rpc search(payload: SearchPayload) -> SearchResultsPayload | ResponseError;
}

type UserRes = CpuFuture<UserPayload, ResponseError>;

type CategoryRes = CpuFuture<CategoryPayload, ResponseError>;
type CategoriesRes = CpuFuture<Vec<CategoryPayload>, ResponseError>;

type ThreadRes = CpuFuture<ThreadPayload, ResponseError>;
type ThreadsRes = CpuFuture<Vec<ThreadPayload>, ResponseError>;

type CommentRes = CpuFuture<CommentPayload, ResponseError>;
type CommentsRes = CpuFuture<Vec<CommentPayload>, ResponseError>;

type SearchRes = CpuFuture<SearchResultsPayload, ResponseError>;

#[macro_export]
macro_rules! impl_service {
    ($s_type:ident, $s_name:ident, $pay:ty, $fut:ident, $res:ty) => {
        type $fut = $res;
        fn $s_name(&self, payload: $pay) -> Self::$fut {
            let cloned_pool = self.db_pool.clone();
            let f = futures::lazy(move || {
                cloned_pool
                    .get()
                    .map_err(|e| {
                        error!("unable to get database connection from the pool: {}", e);
                        ResponseError::InternalServerError
                    })
                    .and_then(|con|
                        $s_type::$s_name(&con, payload)
                            .map(|p| {
                                info!("sending success");
                                p
                            })
                            .map_err(|e| {
                                let ee = e.into();
                                info!("sending error: {}", ee);
                                ee
                            })
                    )
            });
            self.pool.spawn(f)
        }
    }
}

impl FutureService for Server {
    // Users
    impl_service!(users, get_user, GetUserPayload, GetUserFut, UserRes);
    impl_service!(users, add_user, AddUserPayload, AddUserFut, UserRes);
    impl_service!(users, edit_user, EditUserPayload, EditUserFut, UserRes);
    impl_service!(
        users,
        upload_avatar,
        UploadAvatarPayload,
        UploadAvatarFut,
        UserRes
    );

    // Categories
    impl_service!(
        categories,
        get_category,
        GetCategoryPayload,
        GetCategoryFut,
        CategoryRes
    );
    impl_service!(
        categories,
        get_categories,
        GetHiddenPayload,
        GetCategoriesFut,
        CategoriesRes
    );
    impl_service!(
        categories,
        add_category,
        AddCategoryPayload,
        AddCategoryFut,
        CategoryRes
    );
    impl_service!(
        categories,
        edit_category,
        EditCategoryPayload,
        EditCategoryFut,
        CategoryRes
    );
    impl_service!(
        categories,
        hide_category,
        HideCategoryPayload,
        HideCategoryFut,
        CategoryRes
    );

    // Threads
    impl_service!(
        threads,
        get_thread,
        GetThreadPayload,
        GetThreadFut,
        ThreadRes
    );
    impl_service!(
        threads,
        get_threads,
        GetThreadsPayload,
        GetThreadsFut,
        ThreadsRes
    );
    impl_service!(
        threads,
        get_all_threads,
        GetHiddenPayload,
        GetAllThreadsFut,
        ThreadsRes
    );
    impl_service!(
        threads,
        add_thread,
        AddThreadPayload,
        AddThreadFut,
        ThreadRes
    );
    impl_service!(
        threads,
        edit_thread,
        EditThreadPayload,
        EditThreadFut,
        ThreadRes
    );
    impl_service!(
        threads,
        hide_thread,
        HideThreadPayload,
        HideThreadFut,
        ThreadRes
    );

    // Comments
    impl_service!(
        comments,
        get_comment,
        GetCommentPayload,
        GetCommentFut,
        CommentRes
    );
    impl_service!(
        comments,
        get_comments,
        GetCommentsPayload,
        GetCommentsFut,
        CommentsRes
    );
    impl_service!(
        comments,
        get_all_comments,
        GetHiddenPayload,
        GetAllCommentsFut,
        CommentsRes
    );
    impl_service!(
        comments,
        add_comment,
        AddCommentPayload,
        AddCommentFut,
        CommentRes
    );
    impl_service!(
        comments,
        edit_comment,
        EditCommentPayload,
        EditCommentFut,
        CommentRes
    );
    impl_service!(
        comments,
        hide_comment,
        HideCommentPayload,
        HideCommentFut,
        CommentRes
    );

    // Search
    impl_service!(search, search, SearchPayload, SearchFut, SearchRes);
}

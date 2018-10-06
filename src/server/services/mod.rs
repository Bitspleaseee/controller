use super::Server;

use futures_cpupool::CpuFuture;

use datatypes::content::requests::*;
use datatypes::content::responses::*;

mod categories;
mod comments;
mod search;
mod threads;
mod users;

service! {
    rpc get_user(payload: GetUserPayload) -> UserPayload | ContentError;
    rpc add_user(payload: AddUserPayload) -> UserPayload | ContentError;
    rpc edit_user(payload: EditUserPayload) -> UserPayload | ContentError;
    rpc upload_avatar(payload: UploadAvatarPayload) -> UserPayload | ContentError;

    rpc get_category(payload: GetCategoryPayload) -> CategoryPayload | ContentError;
    rpc get_categories(payload: GetHiddenPayload) -> Vec<CategoryPayload> | ContentError;
    rpc add_category(payload: AddCategoryPayload) -> CategoryPayload | ContentError;
    rpc edit_category(payload: EditCategoryPayload) -> CategoryPayload | ContentError;
    rpc hide_category(payload: HideCategoryPayload) -> CategoryPayload | ContentError;

    rpc get_thread(payload: GetThreadPayload) -> ThreadPayload | ContentError;
    rpc get_threads(payload: GetThreadsPayload) -> Vec<ThreadPayload> | ContentError;
    rpc get_all_threads(payload: GetHiddenPayload) -> Vec<ThreadPayload> | ContentError;
    rpc add_thread(payload: AddThreadPayload) -> ThreadPayload | ContentError;
    rpc edit_thread(payload: EditThreadPayload) -> ThreadPayload | ContentError;
    rpc hide_thread(payload: HideThreadPayload) -> ThreadPayload | ContentError;

    rpc get_comment(payload: GetCommentPayload) -> CommentPayload | ContentError;
    rpc get_comments(payload: GetCommentsPayload) -> Vec<CommentPayload> | ContentError;
    rpc get_all_comments(payload: GetHiddenPayload) -> Vec<CommentPayload> | ContentError;
    rpc add_comment(payload: AddCommentPayload) -> CommentPayload | ContentError;
    rpc edit_comment(payload: EditCommentPayload) -> CommentPayload | ContentError;
    rpc hide_comment(payload: HideCommentPayload) -> CommentPayload | ContentError;

    rpc search(payload: SearchPayload) -> SearchResultsPayload | ContentError;
}

type UserRes = CpuFuture<UserPayload, ContentError>;

type CategoryRes = CpuFuture<CategoryPayload, ContentError>;
type CategoriesRes = CpuFuture<Vec<CategoryPayload>, ContentError>;

type ThreadRes = CpuFuture<ThreadPayload, ContentError>;
type ThreadsRes = CpuFuture<Vec<ThreadPayload>, ContentError>;

type CommentRes = CpuFuture<CommentPayload, ContentError>;
type CommentsRes = CpuFuture<Vec<CommentPayload>, ContentError>;

type SearchRes = CpuFuture<SearchResultsPayload, ContentError>;

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
                        ContentError::InternalServerError
                    })
                    .and_then(|con|
                        $s_type::$s_name(&con, payload)
                            .map_err(|e| {
                                let ee = e.into();
                                error!("sending error: {}", ee);
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

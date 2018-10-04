use super::Server;

use futures_cpupool::CpuFuture;
use tarpc::util::Never;
use tarpc::*;

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::{ResponseError, ResponseResult};

mod categories;
mod comments;
mod search;
mod threads;
mod users;

service! {
    rpc get_user(payload: GetUserPayload) -> ResponseResult<UserPayload>;
    rpc add_user(payload: AddUserPayload) -> ResponseResult<UserPayload>;
    rpc edit_user(payload: EditUserPayload) -> ResponseResult<UserPayload>;
    rpc upload_avatar(payload: UploadAvatarPayload) -> ResponseResult<UserPayload>;

    rpc get_category(payload: GetCategoryPayload) -> ResponseResult<CategoryPayload>;
    rpc get_categories(payload: GetCategoriesPayload) -> ResponseResult<Vec<CategoryPayload>>;
    rpc add_category(payload: AddCategoryPayload) -> ResponseResult<CategoryPayload>;
    rpc edit_category(payload: EditCategoryPayload) -> ResponseResult<CategoryPayload>;
    rpc hide_category(payload: HideCategoryPayload) -> ResponseResult<CategoryPayload>;

    rpc get_thread(payload: GetThreadPayload) -> ResponseResult<ThreadPayload>;
    rpc get_threads(payload: GetThreadsPayload) -> ResponseResult<Vec<ThreadPayload>>;
    rpc add_thread(payload: AddThreadPayload) -> ResponseResult<ThreadPayload>;
    rpc edit_thread(payload: EditThreadPayload) -> ResponseResult<ThreadPayload>;
    rpc hide_thread(payload: HideThreadPayload) -> ResponseResult<ThreadPayload>;

    rpc get_comment(payload: GetCommentPayload) -> ResponseResult<CommentPayload>;
    rpc get_comments(payload: GetCommentsPayload) -> ResponseResult<Vec<CommentPayload>>;
    rpc add_comment(payload: AddCommentPayload) -> ResponseResult<CommentPayload>;
    rpc edit_comment(payload: EditCommentPayload) -> ResponseResult<CommentPayload>;
    rpc hide_comment(payload: HideCommentPayload) -> ResponseResult<CommentPayload>;

    rpc search(payload: SearchPayload) -> ResponseResult<SearchResultsPayload>;
}

type UserRes = CpuFuture<ResponseResult<UserPayload>, Never>;

type CategoryRes = CpuFuture<ResponseResult<CategoryPayload>, Never>;
type CategoriesRes = CpuFuture<ResponseResult<Vec<CategoryPayload>>, Never>;

type ThreadRes = CpuFuture<ResponseResult<ThreadPayload>, Never>;
type ThreadsRes = CpuFuture<ResponseResult<Vec<ThreadPayload>>, Never>;

type CommentRes = CpuFuture<ResponseResult<CommentPayload>, Never>;
type CommentsRes = CpuFuture<ResponseResult<Vec<CommentPayload>>, Never>;

type SearchRes = CpuFuture<ResponseResult<SearchResultsPayload>, Never>;

#[macro_export]
macro_rules! impl_service {
    ($s_type:ident, $s_name:ident, $pay:ty, $fut:ident, $res:ty) => {
        type $fut = $res;
        fn $s_name(&self, payload: $pay) -> Self::$fut {
            let cloned_pool = self.db_pool.clone();
            let f = futures::lazy(move || {
                cloned_pool
                    .get()
                    .map(|con| $s_type::$s_name(&con, payload))
                    .or(Ok(Err(ResponseError::InternalServerError)))
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
        GetCategoriesPayload,
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

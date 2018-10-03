use super::super::db;
use super::Server;

use futures_cpupool::CpuFuture;
use tarpc::util::Never;
use tarpc::*;

use datatypes::content::requests::ContentRequest::*;
use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseError;

service! {
    rpc add_user(user: AddUserPayload) -> Result<UserPayload, ResponseError>;
    rpc content_request(request: ContentRequest) -> Result<ContentSuccess, ResponseError>;
}

impl FutureService for Server {
    type AddUserFut = CpuFuture<Result<UserPayload, ResponseError>, Never>;

    // TODO convert internal error into external error instead of
    // returning `None`
    fn add_user(&self, user: AddUserPayload) -> Self::AddUserFut {
        let cloned_pool = self.db_pool.clone();
        let f = futures::lazy(move || {
            cloned_pool
                .get()
                .map(
                    |con| match db::users::insert_user(&con, user.id(), user.username()) {
                        Ok(value) => Ok(value.into()),
                        Err(error) => Err(error.into()),
                    },
                ).or(Ok(Err(ResponseError::InternalServerError)))
        });
        self.pool.spawn(f)
    }

    type ContentRequestFut = CpuFuture<Result<ContentSuccess, ResponseError>, Never>;

    fn content_request(&self, request: ContentRequest) -> Self::ContentRequestFut {
        let cloned_pool = self.db_pool.clone();
        let f = futures::lazy(move || {
            cloned_pool
                .get()
                .map(|con| match request {
                    AddCategory(payload) => match db::categories::insert_category(
                        &con,
                        payload.title(),
                        payload.description(),
                    ) {
                        Ok(value) => Ok(ContentSuccess::Category(value.into())),
                        Err(error) => Err(error.into()),
                    },
                    EditCategory(payload) => {
                        if payload.title().is_some() && payload.description().is_some() {
                            /*db::categories::update_category(
                                &con,
                                payload.id(),
                                payload.title().unwrap(),
                                payload.description().unwrap(),
                            );*/
                        } else if payload.title().is_some() {
                            /*db::categories::update_category_title(
                                &con,
                                payload.id(),
                                payload.title().unwrap(),
                            );*/
                        } else if payload.description().is_some() {
                            /*db::categories::update_category_description(
                                &con,
                                payload.id(),
                                payload.description().unwrap(),
                            );*/
                        }
                        Err(ResponseError::InternalServerError)
                    }
                    HideCategory(payload) => {
                        db::categories::update_category_hidden(&con, payload.id(), payload.hide());
                        Err(ResponseError::InternalServerError)
                    }
                    AddThread(payload) => {
                        //
                        Err(ResponseError::InternalServerError)
                    }
                    EditThread(payload) => {
                        //
                        Err(ResponseError::InternalServerError)
                    }
                    HideThread(payload) => {
                        //
                        Err(ResponseError::InternalServerError)
                    }
                    AddComment(payload) => {
                        //
                        Err(ResponseError::InternalServerError)
                    }
                    EditComment(payload) => {
                        //
                        Err(ResponseError::InternalServerError)
                    }
                    HideComment(payload) => {
                        //
                        Err(ResponseError::InternalServerError)
                    }
                    EditUser(payload) => {
                        db::users::update_user_description(
                            &con,
                            payload.id(),
                            payload.description(),
                        );
                        Err(ResponseError::InternalServerError)
                    }
                    UploadAvatar(payload) => {
                        db::users::update_user_avatar(&con, payload.id(), payload.avatar());
                        Err(ResponseError::InternalServerError)
                    }
                    Search(payload) => {
                        //
                        Err(ResponseError::InternalServerError)
                    }
                }).or(Ok(Err(ResponseError::InternalServerError)))
        });
        self.pool.spawn(f)
    }
}

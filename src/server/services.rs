use super::super::db;
use super::super::types::*;
use super::Server;

use futures_cpupool::CpuFuture;
use tarpc::util::Never;
use tarpc::*;

service! {
    rpc get_user(id: i32) -> Option<User>;
    rpc insert_user(user: NewUser) -> Option<User>;
}

impl FutureService for Server {
    type GetUserFut = CpuFuture<Option<User>, Never>;

    // TODO convert internal error into external error instead of
    // returning `None`
    fn get_user(&self, id: i32) -> Self::GetUserFut {
        let cloned_pool = self.db_pool.clone();
        let f = futures::lazy(move || {
            cloned_pool
                .get()
                .map(|con| {
                    db::users::get_user(&con, id)
                        .map_err(|e| error!("{}", e))
                        .ok()
                }).or(Ok(None))
        });
        self.pool.spawn(f)
    }

    type InsertUserFut = CpuFuture<Option<User>, Never>;

    // TODO convert internal error into external error instead of
    // returning `None`
    fn insert_user(&self, user: NewUser) -> Self::InsertUserFut {
        let cloned_pool = self.db_pool.clone();
        let f = futures::lazy(move || {
            cloned_pool
                .get()
                .map(|con| {
                    db::users::insert_user(&con, user.id, &user.username)
                        .map_err(|e| error!("{}", e))
                        .ok()
                }).or(Ok(None))
        });
        self.pool.spawn(f)
    }
}

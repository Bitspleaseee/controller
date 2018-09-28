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

    fn get_user(&self, id: i32) -> Self::GetUserFut {
        let cloned_pool = self.db_pool.clone();
        self.pool
            .spawn(futures::lazy(move || match cloned_pool.get() {
                Ok(con) => Ok(db::users::get_user_con_pool(&con, id).ok()),
                Err(_) => Ok(None),
            }))
    }

    type InsertUserFut = CpuFuture<Option<User>, Never>;

    fn insert_user(&self, user: NewUser) -> Self::InsertUserFut {
        self.pool.spawn(futures::lazy(move || {
            Ok(db::users::insert_user(user.id, &user.username).ok())
        }))
    }
}

mod services;
use self::services::*;

use failure::ResultExt;
use futures_cpupool::CpuPool;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tarpc::future::server;
use tokio_core::reactor;

use super::db::{setup_connection_pool, DbPool};
use std::net::SocketAddr;

use crate::{IntErrorKind, IntResult};

/// A server which receives requests through tarpc and then performes queries
/// to the MySQL database
#[derive(Clone)]
pub struct Server {
    pool: CpuPool,
    request_count: Arc<AtomicUsize>,
    db_pool: DbPool,
}

impl Server {
    /// Try to make a new server by creating a connection pool to the database
    pub fn try_new(database_url: &str) -> IntResult<Self> {
        let db_pool = setup_connection_pool(database_url)?;

        Ok(Server {
            pool: CpuPool::new_num_cpus(),
            request_count: Arc::new(AtomicUsize::new(1)),
            db_pool,
        })
    }

    /// Run the current server on the given socket address
    pub fn run(self, addr: SocketAddr) -> IntResult<()> {
        let mut reactor = reactor::Core::new().context(IntErrorKind::ServerError)?;

        let (_handle, server) = self
            .listen(addr, &reactor.handle(), server::Options::default())
            .context(IntErrorKind::ServerError)?;

        info!("Starting server on {}", addr);
        reactor.run(server).map_err(|_| IntErrorKind::ServerError)?;
        info!("Shuting down server");
        Ok(())
    }
}

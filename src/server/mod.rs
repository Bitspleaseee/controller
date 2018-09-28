mod services;
use self::services::*;

use futures_cpupool::CpuPool;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tarpc::future::server;
use tarpc::util::FirstSocketAddr;
use tokio_core::reactor;

use super::db::{setup_connection_pool, DbPool};

#[derive(Clone)]
struct Server {
    pool: CpuPool,
    request_count: Arc<AtomicUsize>,
    db_pool: DbPool,
}

impl Server {
    fn new() -> Self {
        Server {
            pool: CpuPool::new_num_cpus(),
            request_count: Arc::new(AtomicUsize::new(1)),
            db_pool: setup_connection_pool(),
        }
    }
}

pub fn start_server() {
    let mut reactor = reactor::Core::new().unwrap();
    let (handle, server) = Server::new()
        .listen(
            "localhost:10000".first_socket_addr(),
            &reactor.handle(),
            server::Options::default(),
        )
        .unwrap();

    println!("Starting server on {}", handle.addr());
    reactor.run(server).unwrap();

    println!("Shuting down server");
}

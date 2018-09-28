#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate fern;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate clap;

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate futures;
extern crate futures_cpupool;
extern crate tarpc;
extern crate tokio_core;

pub mod db;
pub mod error;
pub mod logging;
pub mod server;
pub mod types;

use dotenv::dotenv;
use failure::ResultExt;
use tarpc::util::FirstSocketAddr;

use self::db::categories::*;
use self::db::establish_connection;
use self::db::users::*;
use self::server::Server;

// Include internal error type as Int[ernal]Error
pub use self::error::{Error as IntError, ErrorKind as IntErrorKind};

/// Convenience wrapper around `Result` for the internal
/// [Error](error/struct.Error.html)-type
pub type IntResult<T> = std::result::Result<T, IntError>;

fn run() -> IntResult<()> {
    dotenv().ok();
    // Get verbosity of program from the commandline
    let cmd_arguments = clap::App::new("controller")
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Increases logging verbosity each use for up to 3 times"),
        ).get_matches();

    let verbosity: u64 = cmd_arguments.occurrences_of("verbose");

    logging::setup_logging(verbosity).expect("failed to initialize logging");

    // Get database url and other environment variables
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set as an environment variable or in a '.env' file");

    info!("attempting to connect to database");
    // Clear db
    let conn = establish_connection(&database_url)?;

    info!("clearing database");
    delete_all_users(&conn)?;
    delete_all_categories(&conn)?;

    // Server
    let server = Server::try_new(&database_url)?;
    // TODO setup host addr based on environment variables
    let addr = "localhost:10000"
        .try_first_socket_addr()
        .context(IntErrorKind::ServerError)?;

    info!("attempting to start tarpc server");
    // Runs the server by blocking this thread
    server.run(addr)
}

fn main() {
    if let Err(e) = run() {
        error!("{}", e);
        std::process::exit(1)
    } else {
        std::process::exit(0)
    }
}

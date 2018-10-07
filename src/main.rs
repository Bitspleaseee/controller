#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin)]
#![feature(try_from)]
#![plugin(tarpc_plugins)]

extern crate datatypes;
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
#[macro_use]
extern crate tarpc;
extern crate tokio_core;

#[macro_use]
pub mod macros;
pub mod db;
pub mod error;
pub mod logging;
pub mod server;
pub mod types;

use dotenv::dotenv;
use failure::ResultExt;
use tarpc::util::FirstSocketAddr;

use self::db::categories::delete_all_categories;
use self::db::comments::delete_all_comments;
use self::db::establish_connection;
use self::db::threads::delete_all_threads;
use self::db::users::delete_all_users;
use self::server::Server;

// Include internal error type as Int[ernal]Error
pub use self::error::{Error as IntError, ErrorKind as IntErrorKind};

/// Convenience wrapper around `Result` for the internal
/// [Error](error/struct.Error.html)-type
pub type IntResult<T> = std::result::Result<T, IntError>;

fn run() -> IntResult<()> {
    dotenv().ok();
    // Get program arguments from the commandline
    let cmd_arguments = clap::App::new("controller")
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Increases logging verbosity each use for up to 3 times"),
        ).arg(
            clap::Arg::with_name("clear")
                .short("c")
                .long("clear")
                .multiple(true)
                .help("Clears the database on startup"),
        ).get_matches();

    // Logging
    let verbosity: u64 = cmd_arguments.occurrences_of("verbose");
    logging::setup_logging(verbosity).expect("failed to initialize logging");

    // Get database url and other environment variables
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set as an environment variable or in a '.env' file");
    {
        // Test db connection
        info!("Attempting to connect to database");
        let conn = establish_connection(&database_url)?;

        // Clear db
        let clear: u64 = cmd_arguments.occurrences_of("clear");

        if clear > 0 {
            info!("Clearing database");
            delete_all_users(&conn)?;
            delete_all_categories(&conn)?;
            delete_all_threads(&conn)?;
            delete_all_comments(&conn)?;
        }
    }

    // Server
    let address = match std::env::var("CONTROLLER_ADDRESS") {
        Ok(value) => value,
        Err(_) => {
            warn!("CONTROLLER_ADDRESS is not set, using 'localhost:10000'");
            "localhost:10000".to_string()
        }
    }.try_first_socket_addr()
    .context(IntErrorKind::ServerError)?;

    info!("Attempting to start tarpc server");

    let server = Server::try_new(&database_url)?;
    server.run(address)
}

fn main() {
    if let Err(e) = run() {
        error!("{}", e);
        std::process::exit(1)
    } else {
        std::process::exit(0)
    }
}

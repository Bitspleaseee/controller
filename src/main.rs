#![allow(proc_macro_derive_resolution_fallback)]
#![feature(plugin)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate fern;
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate clap;

extern crate futures;
extern crate futures_cpupool;
extern crate tarpc;
extern crate tokio_core;

pub mod db;
pub mod logging;
pub mod server;
pub mod types;

use self::db::categories::*;
use self::db::users::*;

use log::*;

fn main() {
    // Logging
    let cmd_arguments = clap::App::new("cmd-program")
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Increases logging verbosity each use for up to 3 times"),
        )
        .get_matches();

    let verbosity: u64 = cmd_arguments.occurrences_of("verbose");

    logging::setup_logging(verbosity).expect("failed to initialize logging.");

    info!("Starting program");

    // Clear db
    delete_all_users().expect("Error clearing user table");
    delete_all_categories().expect("Error clearing category table");

    // Server
    server::start_server();
}

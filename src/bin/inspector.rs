#![feature(plugin)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
extern crate futures;
extern crate tokio_core;
#[macro_use]
extern crate serde_derive;
extern crate rustyline;

use futures::Future;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use tarpc::future::client;
use tarpc::future::client::ClientExt;
use tarpc::util::FirstSocketAddr;
use tokio_core::reactor;

use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub id: i32,
    pub username: String,
}

service! {
    rpc get_user(id: i32) -> Option<User>;
    rpc insert_user(user: NewUser) -> Option<User>;
}

const MODES: &[&str] = &["main", "users", "category"];

fn main() {
    let mut reactor = reactor::Core::new().unwrap();

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut mode: String = "main".into();
    loop {
        let readline = rl.readline(&format!("{} >> ", mode));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                // Change between databases ol.
                if line.starts_with("mode") {
                    let new_mode = line.split(char::is_whitespace)
                        .nth(1).unwrap_or("main");
                    if MODES.iter().find(|m| *m == &new_mode).is_some() {
                        mode = new_mode.into();
                    }
                } else {
                    cmd_handler(&mut reactor, &mut mode, &line);
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

fn cmd_handler<'a>(reactor: &mut reactor::Core, mode: &'a str, s: &'a str) {
    let mut it = s.split(char::is_whitespace);
    match it.next().unwrap() {
        "get" => {
            let opt_n = it.next().and_then(|s| s.parse().ok());
            match mode {
                "users" => {
                    opt_n.map(|n| run_get_user(&mut reactor, n));
                },
                _ => {}
            }
        },
        "insert" => {
            let opt_id = it.next().and_then(|s| s.parse().ok());
            let opt_username = it.next();
            match mode {
                "users" => {
                    opt_id.and_then(|id|
                        opt_username.map(|name| {
                            let new_user = NewUser { id, username: name.into() };
                            run_insert_user(&mut reactor, new_user);
                        })
                    );
                },
                _ => {}
            }
        },
        _ => {}
    }
}

fn run_get_user(reactor: &mut reactor::Core, id: i32) {
    let options = client::Options::default().handle(reactor.handle());
    reactor
        .run(
            FutureClient::connect("localhost:10000".first_socket_addr(), options)
                .map_err(tarpc::Error::from)
                .and_then(|client| client.get_user(id))
                .map(|user| match user {
                    Some(value) => println!("The server responded with: {:#?}", value),
                    None => println!("The server responded with: No user"),
                }),
        ).unwrap();
}

fn run_insert_user(reactor: &mut reactor::Core, new_user: NewUser) {
    let options = client::Options::default().handle(reactor.handle());
    reactor
        .run(
            FutureClient::connect("localhost:10000".first_socket_addr(), options)
                .map_err(tarpc::Error::from)
                .and_then(|client| client.insert_user(new_user))
                .map(|user| match user {
                    Some(value) => println!("The server responded with: {:#?}", value),
                    None => println!("The server responded with: No user"),
                }),
        ).unwrap();
}

#![feature(plugin)]
#![feature(try_from)]
#![feature(try_trait)]
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
use std::default::Default;
use std::convert::{TryFrom, TryInto};

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

impl NewUser {
    pub fn new(id: i32, n: impl Into<String>) -> NewUser {
        NewUser {
            id,
            username: n.into()
        }
    }
}

service! {
    rpc get_user(id: i32) -> Option<User>;
    rpc insert_user(user: NewUser) -> Option<User>;
}

pub enum Cmd {
    Get,
    Insert,
}

impl TryFrom<&str> for Cmd {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use self::Cmd::*;
        match s {
            "get" => Ok(Get),
            "insert" => Ok(Insert),
            _ => Err(())
        }
    }
}

pub enum Mode {
    Main,
    Users,
    Categories,
}

impl TryFrom<&str> for Mode {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use self::Mode::*;
        match s {
            "main" => Ok(Main),
            "users" => Ok(Users),
            "categories" => Ok(Categories),
            _ => Err(())
        }
    }
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Main => "main",
            Mode::Users => "users",
            Mode::Categories => "categories"
        }
    }
}

pub struct State {
    mode: Mode
}

impl State {
    pub fn try_set_mode(&mut self, maybe_mode: impl TryInto<Mode>) -> Option<Mode> {
        maybe_mode
            .try_into()
            .ok()
            .map(|new_mode| std::mem::replace(&mut self.mode, new_mode))
    }
}

impl Default for State {
    fn default() -> State {
        State {
            mode: Mode::Main
        }
    }
}


fn main() {
    let mut reactor = reactor::Core::new().unwrap();
    let mut state = State::default();

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(&format!("{} >> ", state.mode.as_str()));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                // Change between databases ol.
                if line.starts_with("mode") {
                    line.split(char::is_whitespace).nth(1)
                        .map(|mode_str| state.try_set_mode(mode_str));
                } else {
                    cmd_handler(&mut reactor, &mut state, &line);
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

fn cmd_handler<'a>(mut reactor: &mut reactor::Core, mut state: &mut State, s: &'a str) -> Result<(), std::option::NoneError> {
    let mut words = s.split(char::is_whitespace);
    let cmd = words.next().and_then(|w| w.try_into().ok())?;
    match state.mode {
        Mode::Users => {
            match cmd {
                Cmd::Get => {
                    let id = words.next().and_then(|w| w.parse().ok())?;
                    run_get_user(&mut reactor, id);
                    Ok(())
                }
                Cmd::Insert => {
                    let id = words.next().and_then(|w| w.parse().ok())?;
                    let username = words.next()?;
                    let new_user = NewUser::new(id, username);
                    run_insert_user(&mut reactor, new_user);
                    Ok(())
                }
            }
        }
        _ => Ok(())
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

#![feature(plugin)]
#![feature(try_from)]
#![feature(try_trait)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
extern crate futures;
extern crate rustyline;
extern crate serde_derive;
extern crate tokio_core;

extern crate datatypes;

use futures::Future;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::convert::{TryFrom, TryInto};
use std::default::Default;
use tarpc::future::client;
use tarpc::future::client::ClientExt;
use tarpc::util::FirstSocketAddr;
use tokio_core::reactor;

use datatypes::content::requests::ContentRequest::*;
use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseError;
use datatypes::valid::fields::*;

service! {
    rpc add_user(user: AddUserPayload) -> Result<UserPayload, ResponseError>;
    rpc content_request(request: ContentRequest) -> Result<ContentSuccess, ResponseError>;
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
            _ => Err(()),
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
            _ => Err(()),
        }
    }
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Main => "main",
            Mode::Users => "users",
            Mode::Categories => "categories",
        }
    }
}

pub struct State {
    mode: Mode,
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
        State { mode: Mode::Main }
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
                    line.split(char::is_whitespace)
                        .nth(1)
                        .map(|mode_str| state.try_set_mode(mode_str));
                } else {
                    cmd_handler(&mut reactor, &state, &line)
                        .unwrap_or_else(|err| println!("Error: {:?}", err));
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

fn cmd_handler<'a>(
    mut reactor: &mut reactor::Core,
    state: &State,
    s: &'a str,
) -> Result<(), std::option::NoneError> {
    let mut words = s.split(char::is_whitespace);
    let cmd = words.next().and_then(|w| w.try_into().ok())?;
    match state.mode {
        Mode::Users => match cmd {
            Cmd::Get => {
                //let id = words.next().and_then(|w| w.parse().ok())?;
                //run_get_user(&mut reactor, id);
                Ok(())
            }
            Cmd::Insert => {
                let id = words.next().and_then(|w| w.parse().ok())?;
                let username = words
                    .next()
                    .and_then(|w| Username::try_from(w.to_string()).ok())?;
                run_add_user(&mut reactor, id, username);
                Ok(())
            }
        },
        Mode::Categories => match cmd {
            Cmd::Get => Ok(()),
            Cmd::Insert => {
                run_add_category(&mut reactor);
                Ok(())
            }
        },
        _ => Ok(()),
    }
}
/*
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
*/
fn run_add_user(reactor: &mut reactor::Core, id: u32, username: Username) {
    let request = AddUserPayload::new(id, username);

    let options = client::Options::default().handle(reactor.handle());
    reactor
        .run(
            FutureClient::connect("localhost:10000".first_socket_addr(), options)
                .map_err(tarpc::Error::from)
                .and_then(|client| client.add_user(request))
                .map(|user| match user {
                    Ok(value) => println!("The server responded with: {:#?}", value),
                    Err(error) => println!("The server responded with error: {}", error),
                }),
        ).unwrap();
}

fn run_add_category(reactor: &mut reactor::Core) {
    let title = Title::try_from("Test cat title".to_string()).expect("Invalid title");
    let description =
        Description::try_from("Test cat description".to_string()).expect("Invalid description");
    let request = AddCategory(AddCategoryPayload::new(title, description));

    let options = client::Options::default().handle(reactor.handle());
    reactor
        .run(
            FutureClient::connect("localhost:10000".first_socket_addr(), options)
                .map_err(tarpc::Error::from)
                .and_then(|client| client.content_request(request))
                .map(|response| match response {
                    Ok(value) => println!("The server responded with: {:#?}", value),
                    Err(error) => println!("The server responded with error: {}", error),
                }),
        ).unwrap();
}

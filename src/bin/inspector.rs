#![feature(plugin)]
#![feature(try_from)]
#![feature(try_trait)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
extern crate futures;
extern crate rustyline;
extern crate serde_derive;

extern crate datatypes;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::fmt::Debug;

use tarpc::sync::client;
use tarpc::sync::client::ClientExt;
use tarpc::util::FirstSocketAddr;

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::valid::fields::*;

service! {
    rpc get_user(payload: GetUserPayload) -> UserPayload | ContentError;
    rpc add_user(payload: AddUserPayload) -> UserPayload | ContentError;
    rpc edit_user(payload: EditUserPayload) -> UserPayload | ContentError;
    rpc upload_avatar(payload: UploadAvatarPayload) -> UserPayload | ContentError;

    rpc get_category(payload: GetCategoryPayload) -> CategoryPayload | ContentError;
    rpc get_categories(payload: GetHiddenPayload) -> Vec<CategoryPayload> | ContentError;
    rpc add_category(payload: AddCategoryPayload) -> CategoryPayload | ContentError;
    rpc edit_category(payload: EditCategoryPayload) -> CategoryPayload | ContentError;
    rpc hide_category(payload: HideCategoryPayload) -> CategoryPayload | ContentError;

    rpc get_thread(payload: GetThreadPayload) -> ThreadPayload | ContentError;
    rpc get_threads(payload: GetThreadsPayload) -> Vec<ThreadPayload> | ContentError;
    rpc get_all_threads(payload: GetHiddenPayload) -> Vec<ThreadPayload> | ContentError;
    rpc add_thread(payload: AddThreadPayload) -> ThreadPayload | ContentError;
    rpc edit_thread(payload: EditThreadPayload) -> ThreadPayload | ContentError;
    rpc hide_thread(payload: HideThreadPayload) -> ThreadPayload | ContentError;

    rpc get_comment(payload: GetCommentPayload) -> CommentPayload | ContentError;
    rpc get_comments(payload: GetCommentsPayload) -> Vec<CommentPayload> | ContentError;
    rpc get_all_comments(payload: GetHiddenPayload) -> Vec<CommentPayload> | ContentError;
    rpc add_comment(payload: AddCommentPayload) -> CommentPayload | ContentError;
    rpc edit_comment(payload: EditCommentPayload) -> CommentPayload | ContentError;
    rpc hide_comment(payload: HideCommentPayload) -> CommentPayload | ContentError;

    rpc search(payload: SearchPayload) -> SearchResultsPayload | ContentError;
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
                    cmd_handler(&state, &line)
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
    state: &State,
    s: &'a str,
) -> Result<(), std::option::NoneError> {
    let mut words = s.split(char::is_whitespace);
    let cmd = words.next().and_then(|w| w.try_into().ok())?;
    match state.mode {
        Mode::Users => match cmd {
            Cmd::Get => {
                let id = words.next().and_then(|w| w.parse().ok())?;
                run_get_user(id);
                Ok(())
            }
            Cmd::Insert => {
                let id = words.next().and_then(|w| w.parse().ok())?;
                let username = words
                    .next()
                    .and_then(|w| Username::try_from(w.to_string()).ok())?;
                run_add_user(id, username);
                Ok(())
            }
        },
        Mode::Categories => match cmd {
            Cmd::Get => {
                let id = words.next().and_then(|w| w.parse().ok())?;
                run_get_category(id);
                Ok(())
            },
            Cmd::Insert => {
                run_add_category();
                Ok(())
            }
        },
        _ => Ok(()),
    }
}

// Users

fn run_get_user(id: u32) {
    // Build request
    let request = GetUserPayload {
        id: id.into()
    };

    // Call
    run_client_action(move |client| client.get_user(request));
}

fn run_add_user(id: u32, username: Username) {
    // Build request
    let request = AddUserPayload {
        id: id.into(),
        username,
    };

    // Call
    run_client_action(|client| client.add_user(request));
}

// Categories

fn run_get_category(id: u32) {
    let request = GetCategoryPayload {
        id: id.into()
    };

    run_client_action(|client| client.get_category(request));
}

fn run_add_category() {
    // Build request
    let title = Title::try_from("Test cat title".to_string()).expect("Invalid title");
    let description =
        Description::try_from("Test cat description".to_string()).expect("Invalid description");
    let request = AddCategoryPayload { title, description };

    // Call
    run_client_action(|client| client.add_category(request));
}

// Connect to server
fn connect() -> Option<SyncClient> {
    let options = client::Options::default();
    let addr = "localhost:10000".first_socket_addr();

    SyncClient::connect(addr, options)
        .map_err(|e| println!("Error connecting: {:#?}", e))
        .ok()
}

// Run a action on the server and print the result
fn run_client_action<T, E, F>(f: F)
    where
        T: Debug,
        E: Debug,
        F: FnOnce(SyncClient) -> Result<T, E>
    {
    if let Some(client) = connect() {
        match f(client) {
            Ok(value) => println!("The server responded with: {:#?}", value),
            Err(error) => println!("The server responded with error: {:#?}", error),
        }
    }
}



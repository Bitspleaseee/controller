#![feature(plugin)]
#![feature(try_from)]
#![feature(try_trait)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
extern crate datatypes;
extern crate futures;
extern crate rustyline;
extern crate serde_derive;
#[macro_use]
extern crate failure;

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

use failure::Fallible;
macro_rules! enum_str {
    {
        $( #[ $( $attr:meta ),* ] )*
        pub enum $name:ident {
            $( $sub:ident => $sub_str:expr ),*
        }
    } => {
        $( #[ $( $attr ),* ] )*
        pub enum $name {
            $( $sub ),*
        }
        impl TryFrom<&str> for $name {
            type Error = failure::Error;
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                match s {
                    $( $sub_str => Ok($name::$sub), )*
                    s => Err(format_err!("Invalid {}: '{}'", stringify!($name), s))
                }
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $( $name::$sub => write!(f, "{}", $sub_str), )*
                }
            }
        }
    }
}

enum_str! {
    #[derive(Copy, Clone)]
    pub enum Cmd {
        Get => "get",
        GetAll => "get-all",
        Insert => "insert",
        Hide => "hide",
        Edit => "edit",
        Delete => "delete"
    }
}

enum_str! {
    #[derive(Copy, Clone)]
    pub enum Mode {
        Main => "main",
        Users => "users",
        Categories => "categories",
        Threads => "threads"
    }
}

fn cmd_handler<'a>(state: &State, s: &'a str) -> Fallible<()> {
    let mut args = s.split(char::is_whitespace);
    let cmd = args
        .next()
        .ok_or(format_err!("Missing argument <cmd>"))
        .and_then(|w| w.try_into())?;
    match (state.mode, cmd) {
        (Mode::Users, Cmd::Get) => run_get_user(args),
        (Mode::Users, Cmd::Insert) => run_insert_user(args),
        (Mode::Users, Cmd::Edit) => run_edit_user(args),
        (Mode::Categories, Cmd::Get) => run_get_category(args),
        (Mode::Categories, Cmd::GetAll) => run_get_all_categories(args),
        (Mode::Categories, Cmd::Insert) => run_insert_category(args),
        (Mode::Categories, Cmd::Edit) => run_edit_category(args),
        (Mode::Categories, Cmd::Hide) => run_hide_category(args),
        (m, c) => Err(format_err!(
            "Unimplemented command '{}' for mode '{}'",
            c,
            m
        )),
    }
}

macro_rules! get_next_field {
    ($args:ident, $name:ident) => {
        $args
            .next()
            .ok_or(format_err!("Missing argument <{}>", stringify!($name)))
            .and_then(|s| {
                s.to_owned()
                    .try_into()
                    .map_err(|_| format_err!("Invalid <{}>", stringify!($name)))
            })
    };
}

macro_rules! get_next_id {
    ($args:ident, $inner:ty => $name:ident) => {
        $args
            .next()
            .ok_or(format_err!("Missing argument <{}>", stringify!($name)))
            .and_then(|s| {
                s.parse::<$inner>()
                    .map(|n| n.into())
                    .map_err(|_| format_err!("Invalid <{}>", stringify!($name)))
            })
    };
}

// User

fn run_get_user<'a>(mut args: impl Iterator<Item = &'a str>) -> Fallible<()> {
    let id = get_next_id!(args, u32 => user_id)?;
    let payload = GetUserPayload { id };

    run_client_action(|client| client.get_user(payload));
    Ok(())
}

fn run_insert_user<'a>(mut args: impl Iterator<Item = &'a str>) -> Fallible<()> {
    let id = get_next_id!(args, u32 => user_id)?;
    let username = get_next_field!(args, username)?;
    let payload = AddUserPayload { id, username };

    run_client_action(|client| client.add_user(payload));
    Ok(())
}

fn run_edit_user<'a>(mut args: impl Iterator<Item = &'a str>) -> Fallible<()> {
    let id = get_next_id!(args, u32 => user_id)?;
    let description = get_next_field!(args, description).ok();
    let avatar = get_next_field!(args, avatar).ok();

    let payload = EditUserPayload {
        id,
        description,
        avatar,
    };

    run_client_action(|client| client.edit_user(payload));
    Ok(())
}

// Category

fn run_get_category<'a>(mut args: impl Iterator<Item = &'a str>) -> Fallible<()> {
    let id = get_next_id!(args, u32 => user_id)?;

    let payload = GetCategoryPayload {
        id,
        include_hidden: true,
    };

    run_client_action(|client| client.get_category(payload));
    Ok(())
}

fn run_get_all_categories<'a>(mut _args: impl Iterator<Item = &'a str>) -> Fallible<()> {
    let payload = GetHiddenPayload {
        include_hidden: true,
    };

    run_client_action(|client| client.get_all_categories(payload));
    Ok(())
}

fn run_insert_category<'a>(mut args: impl Iterator<Item = &'a str>) -> Fallible<()> {
    let title = get_next_field!(args, title)?;
    let description = get_next_field!(args, description)?;

    let payload = AddCategoryPayload { title, description };

    run_client_action(|client| client.add_category(payload));
    Ok(())
}

fn run_edit_category<'a>(mut args: impl Iterator<Item = &'a str>) -> Fallible<()> {
    let id = get_next_id!(args, u32 => category_id)?;
    let title = get_next_field!(args, title).ok();
    let description = get_next_field!(args, description).ok();

    let payload = EditCategoryPayload {
        id,
        title,
        description,
    };

    run_client_action(|client| client.edit_category(payload));
    Ok(())
}

fn run_hide_category<'a>(mut args: impl Iterator<Item = &'a str>) -> Fallible<()> {
    let id = get_next_id!(args, u32 => category_id)?;

    let payload = HideCategoryPayload { id, hide: true };

    run_client_action(|client| client.hide_category(payload));
    Ok(())
}

service! {
    rpc get_user(payload: GetUserPayload) -> UserPayload | ContentError;
    rpc add_user(payload: AddUserPayload) -> UserPayload | ContentError;
    rpc edit_user(payload: EditUserPayload) -> UserPayload | ContentError;

    rpc get_category(payload: GetCategoryPayload) -> CategoryPayload | ContentError;
    rpc get_all_categories(payload: GetHiddenPayload) -> Vec<CategoryPayload> | ContentError;
    rpc add_category(payload: AddCategoryPayload) -> CategoryPayload | ContentError;
    rpc edit_category(payload: EditCategoryPayload) -> CategoryPayload | ContentError;
    rpc hide_category(payload: HideCategoryPayload) -> CategoryPayload | ContentError;

    rpc get_thread(payload: GetThreadPayload) -> ThreadPayload | ContentError;
    rpc get_threads_in_category(payload: GetThreadsPayload) -> Vec<ThreadPayload> | ContentError;
    rpc get_all_threads(payload: GetHiddenPayload) -> Vec<ThreadPayload> | ContentError;
    rpc add_thread(payload: AddThreadPayload) -> ThreadPayload | ContentError;
    rpc edit_thread(payload: EditThreadPayload) -> ThreadPayload | ContentError;
    rpc hide_thread(payload: HideThreadPayload) -> ThreadPayload | ContentError;

    rpc get_comment(payload: GetCommentPayload) -> CommentPayload | ContentError;
    rpc get_comments_in_thread(payload: GetCommentsPayload) -> Vec<CommentPayload> | ContentError;
    rpc get_all_comments(payload: GetHiddenPayload) -> Vec<CommentPayload> | ContentError;
    rpc add_comment(payload: AddCommentPayload) -> CommentPayload | ContentError;
    rpc edit_comment(payload: EditCommentPayload) -> CommentPayload | ContentError;
    rpc hide_comment(payload: HideCommentPayload) -> CommentPayload | ContentError;

    rpc search(payload: SearchPayload) -> SearchResultsPayload | ContentError;
}

// Connect to server
fn connect() -> Option<SyncClient> {
    let options = client::Options::default();
    let addr = "localhost:10000".first_socket_addr();

    SyncClient::connect(addr, options).ok()
}

// Run a action on the server and print the result
fn run_client_action<T, E, F>(f: F)
where
    T: Debug,
    E: Debug,
    F: FnOnce(SyncClient) -> Result<T, E>,
{
    if let Some(client) = connect() {
        match f(client) {
            Ok(value) => println!("The server responded with: {:#?}", value),
            Err(error) => println!("The server responded with error: {:#?}", error),
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
        let readline = rl.readline(&format!("{} >> ", state.mode));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                // Change between databases ol.
                if line.starts_with("mode") {
                    line.split(char::is_whitespace)
                        .nth(1)
                        .map(|mode_str| state.try_set_mode(mode_str));
                } else {
                    cmd_handler(&state, &line).unwrap_or_else(|err| println!("Error: {}", err));
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

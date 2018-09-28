#![feature(plugin)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
extern crate futures;
extern crate tokio_core;
#[macro_use]
extern crate serde_derive;

use futures::Future;
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

fn main() {
    let mut reactor = reactor::Core::new().unwrap();

    loop {
        // Read command
        println!("Get: g");
        println!("Insert: i");
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        cmd = cmd.trim().to_string();

        if cmd == "g" {
            // Get

            // Read id
            println!("id:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read line");
            let id = id.trim().parse().expect("Not a number");

            // Send
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
                )
                .unwrap();
        } else if cmd == "i" {
            // Insert

            // Read id
            println!("id:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("Failed to read line");
            let id = id.trim().parse().expect("Not a number");

            // Read username
            println!("username:");
            let mut username = String::new();
            io::stdin()
                .read_line(&mut username)
                .expect("Failed to read line");
            username = username.trim().to_string();

            let new_user = NewUser { id, username };

            // Send
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
                )
                .unwrap();
        }
    }
}

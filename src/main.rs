extern crate diesel;
extern crate dotenv;

mod db;

fn main() {
    println!("Starting");

    db::establish_connection();
}

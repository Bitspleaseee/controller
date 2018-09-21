#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;

fn main() {
    println!("Starting");

    db::delete_all_users();
    db::insert_user(1, "TestUser_1");
    db::insert_user(2, "TestUser_2");
    db::insert_user(3, "TestUser_3");

    db::delete_user(2);

    let user = db::get_user(3);
    println!("{:#?}", user);
}

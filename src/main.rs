#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;

fn main() {
    // Clear db
    db::delete_all_users().expect("Error clearing user table");

    // Add some users
    db::insert_user(1, "TestUser_1").expect("Error adding test user 1");
    db::insert_user(2, "TestUser_2").expect("Error adding test user 2");
    db::insert_user(3, "TestUser_3").expect("Error adding test user 3");

    // Delete user 2
    db::delete_user(2).expect("Error deleting test user 2");

    // Get user 2
    match db::get_user(2).unwrap_or_else(|_| panic!("Error getting test user 2")) {
        Some(user) => println!("{:#?}", user),
        None => println!("Failed to find test user 2"),
    };

    // Get user 3
    let mut user3 = match db::get_user(3).unwrap_or_else(|_| panic!("Error getting test user 3")) {
        Some(user) => user,
        None => panic!("Failed to find test user 3"),
    };
    println!("{:#?}", user3);

    // Update user 3
    user3.description = Some("This is a test user".to_string());
    db::update_user(&user3).expect("Error updating test user 3");

    // Get user 3
    match db::get_user(3).unwrap_or_else(|_| panic!("Error getting test user 3")) {
        Some(user) => println!("{:#?}", user),
        None => println!("Failed to find test user 3"),
    };
}

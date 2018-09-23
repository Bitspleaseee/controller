#![allow(proc_macro_derive_resolution_fallback)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
use self::db::categories::*;
use self::db::users::*;

fn main() {
    // ========== Users ==========
    println!("\n\n");
    delete_all_users().expect("Error clearing user table");

    // Add test users
    insert_user(1, "TestUser_1").expect("Error adding test user 1");
    insert_user(2, "TestUser_2").expect("Error adding test user 2");
    let user3 = insert_user(3, "TestUser_3").expect("Error adding test user 3");

    // Delete user 2
    delete_user(2).expect("Error deleting test user 2");

    // Get user 2
    match get_user(2) {
        Ok(_) => panic!("Test user 2 should not exist"),
        Err(error) => match error {
            db::Error::NotFound => println!("Failed to find test user 2"),
            _ => panic!("Error getting test user 2"),
        },
    };

    // Get user 1
    let mut user1 = get_user(1).expect("Error getting test user 1");
    println!("{:#?}", user1);

    // Update user 1
    user1.description = Some("This is a test user".to_string());
    let user1 = update_user(&user1).expect("Error updating test user 1");

    // Print user 1
    println!("{:#?}", user1);

    // Update user 3
    update_user_username(user3.id, "NewUser_3_Username")
        .expect("Error updating username for user 3");

    update_user_description(user3.id, "New user 3 description")
        .expect("Error updating description for user 3");

    update_user_avatar(user3.id, "A78asd8").expect("Error updating avatar for user 3");

    let user3 = get_user(3).expect("Error getting test user 3");
    println!("{:#?}", user3);

    // ========== Categories ==========
    println!("\n\n");
    delete_all_categories().expect("Error clearing category table");

    // Add test categories
    let category1 = insert_category("Test category 1", "Used for testing")
        .expect("Error adding test category 1");
    let category2 = insert_category("Test category 2", "Used for testing")
        .expect("Error adding test category 2");
    let category3 = insert_category("Test category 3", "Used for testing")
        .expect("Error adding test category 3");

    // Get category 2
    let mut category2 = get_category(category2.id).expect("Error getting test category 2");
    println!("{:#?}", category2);

    // Update category 2
    category2.title = "New category 2 title".to_string();
    category2.description = "Used for some more testing".to_string();
    update_category(&category2).expect("Error updating test category 2");

    // Update 1 and 3
    update_category_description(category1.id, "This is category 1")
        .expect("Error updating description for category 1");

    update_category_title(category3.id, "New category 3 title")
        .expect("Error updating title for category 3");

    update_category_hidden(category3.id, true).expect("Error updating title for category 3");

    // Get all categories
    let categories = get_all_categories(false).expect("Error getting all categories");
    for category in categories {
        println!("{:#?}", category);
    }

    // Get all categories including hidden
    let categories =
        get_all_categories(true).expect("Error getting all categories, including hidden");
    for category in categories {
        println!("{:#?}", category);
    }
}

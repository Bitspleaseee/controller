use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use self::models::{User};

pub fn establish_connection() -> MysqlConnection {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_user(user_id: i32, username: &str) -> User {
    use self::schema::users::dsl::{users, id};

    let connection = establish_connection();

    let new_user = User {
        id: user_id,
        username: username.to_string(),
        description: None,
        avatar: None,
    };

    println!("Inserting user: {}", new_user.username);
    diesel::insert_into(users)
        .values(&new_user)
        .execute(&connection)
        .expect("Error inserting user");

    users.order(id.desc()).first(&connection).unwrap()
}


pub fn get_user(user_id: i32) -> User {
    use self::schema::users::dsl::*;

    let connection = establish_connection();

    println!("Getting user: {}", user_id);
    users.filter(id.eq(user_id))
        .first(&connection)
        .expect("Error getting user")
}

pub fn delete_user(user_id: i32) -> usize  {
    use self::schema::users::dsl::*;

    let connection = establish_connection();

     let num_deleted = diesel::delete(users)
        .filter(id.eq(user_id))
        .execute(&connection)
        .expect("Error deleting user");
    
     println!("Deleted {} rows", num_deleted);

    num_deleted
}

pub fn delete_all_users() -> usize  {
    use self::schema::users::dsl::*;

    let connection = establish_connection();

     let num_deleted = diesel::delete(users)
        .execute(&connection)
        .expect("Error deleting users");
    
     println!("Deleted {} rows", num_deleted);

    num_deleted
}

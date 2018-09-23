use super::schema::*;

#[derive(Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub hidden: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "categories"]
pub struct NewCategory {
    pub title: String,
    pub description: String,
}

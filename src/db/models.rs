use super::schema::users;

#[derive(Queryable, Insertable, AsChangeset, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

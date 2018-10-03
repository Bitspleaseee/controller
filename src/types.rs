use super::db::schema::*;

use datatypes::content::responses::*;
use datatypes::valid::fields::*;
use std::convert::TryFrom;

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: u32,
    pub username: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}
impl Into<UserPayload> for User {
    fn into(self) -> UserPayload {
        UserPayload::new(
            self.id,
            Username::try_from(self.username).unwrap(),
            match self.description {
                None => None,
                Some(d) => Description::try_from(d).ok(),
            },
            self.avatar,
        )
    }
}

#[derive(Queryable, Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "categories"]
pub struct Category {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub hidden: bool,
}
impl Into<CategoryPayload> for Category {
    fn into(self) -> CategoryPayload {
        CategoryPayload::new(
            self.id,
            Title::try_from(self.title).unwrap(),
            Description::try_from(self.description).unwrap(),
        )
    }
}

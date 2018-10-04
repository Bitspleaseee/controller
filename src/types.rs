use super::db::schema::*;

use datatypes::content::responses::*;
use datatypes::valid::fields::*;
use datatypes::valid::ids::*;
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
        UserPayload {
            id: UserId::try_from(self.id).unwrap(),
            username: Username::try_from(self.username).unwrap(),
            description: match self.description {
                None => None,
                Some(d) => Description::try_from(d).ok(),
            },
            avatar: self.avatar,
        }
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
        CategoryPayload {
            id: CategoryId::try_from(self.id).unwrap(),
            title: Title::try_from(self.title).unwrap(),
            description: Description::try_from(self.description).unwrap(),
            hidden: self.hidden,
        }
    }
}

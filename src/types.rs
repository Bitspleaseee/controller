use super::db::schema::*;

use datatypes::content::responses::*;
use datatypes::valid::ValidationError;

use chrono::naive::NaiveDateTime;
use std::convert::TryInto;

#[derive(Queryable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: u32,
    pub username: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

impl TryInto<UserPayload> for User {
    type Error = ValidationError;
    fn try_into(self) -> Result<UserPayload, Self::Error> {
        let username = self.username.try_into()?;
        Ok(UserPayload {
            id: self.id.into(),
            username,
            description: self.description.and_then(|d| d.try_into().ok()),
            avatar: self.avatar,
        })
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

impl TryInto<CategoryPayload> for Category {
    type Error = ValidationError;
    fn try_into(self) -> Result<CategoryPayload, Self::Error> {
        let title = self.title.try_into()?;
        let description = self.description.try_into()?;
        Ok(CategoryPayload {
            id: self.id.into(),
            title,
            description,
            hidden: self.hidden,
        })
    }
}

#[derive(Queryable, Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "threads"]
pub struct Thread {
    pub id: u32,
    pub category_id: u32,
    pub user_id: u32,
    pub title: String,
    pub description: String,
    pub timestamp: NaiveDateTime,
    pub hidden: bool,
}

impl TryInto<ThreadPayload> for Thread {
    type Error = ValidationError;
    fn try_into(self) -> Result<ThreadPayload, Self::Error> {
        let title = self.title.try_into()?;
        let description = self.description.try_into()?;
        Ok(ThreadPayload {
            id: self.id.into(),
            category_id: self.category_id.into(),
            user_id: self.user_id.into(),
            title,
            description,
            timestamp: self.timestamp,
            hidden: self.hidden,
        })
    }
}

#[derive(Queryable, Debug, AsChangeset, Serialize, Deserialize)]
#[table_name = "comments"]
pub struct Comment {
    pub id: u32,
    pub thread_id: u32,
    pub parent_id: Option<u32>,
    pub user_id: u32,
    pub content: String,
    pub timestamp: NaiveDateTime,
    pub hidden: bool,
}

impl TryInto<CommentPayload> for Comment {
    type Error = ValidationError;
    fn try_into(self) -> Result<CommentPayload, Self::Error> {
        let content = self.content.try_into()?;
        Ok(CommentPayload {
            id: self.id.into(),
            thread_id: self.thread_id.into(),
            parent_id: self.parent_id.and_then(|pid| pid.try_into().ok()),
            user_id: self.user_id.into(),
            content,
            timestamp: self.timestamp,
            hidden: self.hidden,
        })
    }
}

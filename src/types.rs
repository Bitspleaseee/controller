use crate::db::schema::*;

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::valid::ValidationError;

use chrono::naive::NaiveDateTime;
use std::convert::TryInto;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Identifiable, AsChangeset, Debug)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: u32,
    pub description: Option<String>,
    pub avatar: Option<String>,
}

impl From<EditUserPayload> for UpdateUser {
    fn from(p: EditUserPayload) -> UpdateUser {
        UpdateUser {
            id: *p.id,
            description: p.description.map(|s| s.into_inner()),
            avatar: p.avatar,
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct InsertUser {
    pub id: u32,
    pub username: String,
}

impl From<AddUserPayload> for InsertUser {
    fn from(p: AddUserPayload) -> InsertUser {
        InsertUser {
            id: *p.id,
            username: p.username.into_inner(),
        }
    }
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
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

#[derive(Identifiable, AsChangeset, Debug)]
#[table_name = "categories"]
pub struct UpdateCategory {
    pub id: u32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub hidden: Option<bool>,
}

impl From<EditCategoryPayload> for UpdateCategory {
    fn from(p: EditCategoryPayload) -> UpdateCategory {
        UpdateCategory {
            id: *p.id,
            title: p.title.map(|t| t.into_inner()),
            description: p.description.map(|d| d.into_inner()),
            hidden: None,
        }
    }
}

impl From<HideCategoryPayload> for UpdateCategory {
    fn from(p: HideCategoryPayload) -> UpdateCategory {
        UpdateCategory {
            id: *p.id,
            title: None,
            description: None,
            hidden: Some(p.hide),
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name = "categories"]
pub struct InsertCategory {
    pub title: String,
    pub description: String,
}

impl From<AddCategoryPayload> for InsertCategory {
    fn from(p: AddCategoryPayload) -> InsertCategory {
        InsertCategory {
            title: p.title.into_inner(),
            description: p.description.into_inner(),
        }
    }
}

#[derive(Identifiable, Associations, Queryable, Debug, Serialize, Deserialize)]
#[belongs_to(Category)]
#[belongs_to(User)]
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

#[derive(Identifiable, AsChangeset, Debug)]
#[table_name = "threads"]
pub struct UpdateThread {
    pub id: u32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub hidden: Option<bool>,
}

impl From<EditThreadPayload> for UpdateThread {
    fn from(p: EditThreadPayload) -> UpdateThread {
        UpdateThread {
            id: *p.id,
            title: p.title.map(|t| t.into_inner()),
            description: p.description.map(|d| d.into_inner()),
            hidden: None,
        }
    }
}

impl From<HideThreadPayload> for UpdateThread {
    fn from(p: HideThreadPayload) -> UpdateThread {
        UpdateThread {
            id: *p.id,
            title: None,
            description: None,
            hidden: Some(p.hide),
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name = "threads"]
pub struct InsertThread {
    pub category_id: u32,
    pub user_id: u32,
    pub title: String,
    pub description: String,
}

impl From<AddThreadPayload> for InsertThread {
    fn from(p: AddThreadPayload) -> InsertThread {
        InsertThread {
            category_id: *p.category_id,
            user_id: *p.user_id,
            title: p.title.into_inner(),
            description: p.description.into_inner(),
        }
    }
}

#[derive(Identifiable, Associations, Queryable, Debug, Serialize, Deserialize)]
#[belongs_to(Thread)]
#[belongs_to(User)]
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
            parent_id: self.parent_id.map(|pid| pid.into()),
            user_id: self.user_id.into(),
            content,
            timestamp: self.timestamp,
            hidden: self.hidden,
        })
    }
}

#[derive(Identifiable, AsChangeset, Debug)]
#[table_name = "comments"]
pub struct UpdateComment {
    pub id: u32,
    pub content: Option<String>,
    pub hidden: Option<bool>,
}

impl From<EditCommentPayload> for UpdateComment {
    fn from(p: EditCommentPayload) -> UpdateComment {
        UpdateComment {
            id: *p.id,
            content: Some(p.content.into_inner()),
            hidden: None,
        }
    }
}

impl From<HideCommentPayload> for UpdateComment {
    fn from(p: HideCommentPayload) -> UpdateComment {
        UpdateComment {
            id: *p.id,
            content: None,
            hidden: Some(p.hide),
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name = "comments"]
pub struct InsertComment {
    pub thread_id: u32,
    pub user_id: u32,
    pub parent_id: Option<u32>,
    pub content: String,
}

impl From<AddCommentPayload> for InsertComment {
    fn from(p: AddCommentPayload) -> InsertComment {
        InsertComment {
            thread_id: *p.thread_id,
            user_id: *p.user_id,
            parent_id: p.parent_id.map(|i| *i),
            content: p.content.into_inner(),
        }
    }
}

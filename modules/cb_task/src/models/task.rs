use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

use crab_rocket_utils::time::get_e8_time;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    id: i32,
    title: String,
    content: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    user_id: Option<i32>,
}

impl Task {
    pub fn new(id: i32, title: String, content: Option<String>, user_id: Option<i32>) -> Self {
        Task {
            id,
            title,
            content,
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
            user_id,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &Option<String> {
        &self.content
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: Option<String>) {
        self.content = content;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }

    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {},\n title: {},\n content: {},\n created_at: {},\n updated_at: {}\n",
            self.id,
            self.title,
            self.content.clone().unwrap(),
            self.created_at.unwrap(),
            self.updated_at.unwrap()
        )
    }
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::tasks)]
pub struct NewTask {
    title: String,
    content: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    user_id: Option<i32>,
}

impl NewTask {
    pub fn new(
        title: String,
        content: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
        user_id: Option<i32>,
    ) -> Self {
        NewTask {
            title,
            content,
            created_at,
            updated_at,
            user_id,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &Option<String> {
        &self.content
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: Option<String>) {
        self.content = content;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }

    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::tasks)]
pub struct PatchTask {
    title: String,
    content: Option<String>,
    user_id: Option<i32>,
}

impl PatchTask {
    pub fn new(title: String, content: Option<String>, user_id: Option<i32>) -> Self {
        PatchTask {
            title,
            content,
            user_id,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &Option<String> {
        &self.content
    }

    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: Option<String>) {
        self.content = content;
    }

    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }
}

impl Into<PatchTask> for Task {
    fn into(self) -> PatchTask {
        PatchTask {
            title: self.title,
            content: self.content,
            user_id: self.user_id,
        }
    }
}

impl Into<NewTask> for Task {
    fn into(self) -> NewTask {
        NewTask {
            title: self.title,
            content: self.content,
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
            user_id: self.user_id,
        }
    }
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::tasks)]
pub struct PutTask {
    id: i32,
    title: String,
    content: Option<String>,
    updated_at: Option<chrono::NaiveDateTime>,
    user_id: Option<i32>,
}

impl PutTask {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &Option<String> {
        &self.content
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_content(&mut self, content: Option<String>) {
        self.content = content;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }

    pub fn set_user_id(&mut self, user_id: Option<i32>) {
        self.user_id = user_id;
    }

    pub fn new(
        id: i32,
        title: String,
        content: Option<String>,
        updated_at: Option<chrono::NaiveDateTime>,
        user_id: Option<i32>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            updated_at,
            user_id,
        }
    }
}

impl Into<PutTask> for Task {
    fn into(self) -> PutTask {
        PutTask {
            id: self.id,
            title: self.title,
            content: self.content,
            updated_at: self.updated_at,
            user_id: self.user_id,
        }
    }
}

impl Into<PatchTask> for PutTask {
    fn into(self) -> PatchTask {
        PatchTask {
            title: self.title,
            content: self.content,
            user_id: self.user_id,
        }
    }
}

impl Into<NewTask> for PutTask {
    fn into(self) -> NewTask {
        NewTask {
            title: self.title,
            content: self.content,
            created_at: Some(get_e8_time()),
            updated_at: self.updated_at,
            user_id: self.user_id,
        }
    }
}

impl Into<Task> for PutTask {
    fn into(self) -> Task {
        Task {
            id: self.id,
            title: self.title,
            content: self.content,
            created_at: Some(get_e8_time()),
            updated_at: self.updated_at,
            user_id: self.user_id,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_task_new() {
        let task = super::Task::new(1, "title".to_string(), "content".to_string().into(), Some(4));
        println!("{task}");

        let fixed_dt = crab_rocket_utils::time::get_e8_time();
        println!("{:?}", fixed_dt);
    }
}

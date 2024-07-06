use std::fmt::Display;

use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::task_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    task_id: i32,
    title: String,
    content: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    user_id: Option<i32>,
}

impl Task {
    pub fn task_id(&self) -> i32 {
        self.task_id
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

    pub fn set_task_id(&mut self, task_id: i32) {
        self.task_id = task_id;
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

    pub fn new(
        task_id: i32,
        title: String,
        content: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
        user_id: Option<i32>,
    ) -> Self {
        Self {
            task_id,
            title,
            content,
            created_at,
            updated_at,
            user_id,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "task_id: {},\n title: {},\n content: {},\n created_at: {},\n updated_at: {}\n",
            self.task_id,
            self.title,
            self.content.clone().unwrap(),
            self.created_at.unwrap(),
            self.updated_at.unwrap()
        )
    }
}

pub struct TaskList(Vec<Task>);

impl From<Vec<Task>> for TaskList {
    fn from(tasks: Vec<Task>) -> Self {
        TaskList(tasks)
    }
}

impl Display for TaskList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tasks_as_string: Vec<String> = self.0.iter().map(|task| task.to_string()).collect();
        write!(f, "[{}]", tasks_as_string.join(", "))
    }
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::task_table)]
pub struct NewTask {
    title: String,
    content: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
    user_id: Option<i32>,
}

impl NewTask {
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

    pub fn new(
        title: String,
        content: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
        user_id: Option<i32>,
    ) -> Self {
        Self {
            title,
            content,
            created_at,
            updated_at,
            user_id,
        }
    }
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::task_table)]
pub struct PatchTask {
    title: String,
    content: Option<String>,
    user_id: Option<i32>,
}

impl PatchTask {
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

    pub fn new(title: String, content: Option<String>, user_id: Option<i32>) -> Self {
        Self {
            title,
            content,
            user_id,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_task_new() {
        let task = super::Task {
            task_id: 1,
            title: "title".to_string(),
            content: "content".to_string().into(),
            created_at: Some(crab_rocket_utils::time::get_e8_time()),
            updated_at: Some(crab_rocket_utils::time::get_e8_time()),
            user_id: Some(1),
        };
        println!("{task}");

        let fixed_dt = crab_rocket_utils::time::get_e8_time();
        println!("{:?}", fixed_dt);
    }
}

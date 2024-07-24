use std::fmt::Display;

use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crab_rocket_utils::time::get_e8_time;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::task_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub task_id: i32,
    pub title: String,
    pub content: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub user_id: Option<i32>,
}

impl Task {
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
pub struct PostTask {
    pub title: String,
    pub content: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub user_id: Option<i32>,
}

impl PostTask {
    pub fn demo() -> Self {
        Self {
            title: "Post Task".to_owned(),
            content: None,
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
            user_id: None,
        }
    }
}

impl PostTask {
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
    pub title: String,
    pub content: Option<String>,
    pub user_id: Option<i32>,
}

impl PatchTask {
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

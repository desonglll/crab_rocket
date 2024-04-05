use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::utils::time::get_e8_time;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask {
    pub title: String,
    pub content: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Task {
    pub fn new(id: i32, title: String, content: Option<String>) -> Self {
        Task {
            id,
            title,
            content,
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
            // created_at: Some(chrono::Local::now().with_timezone(&Shanghai).naive_utc()),
            // updated_at: Some(chrono::Local::now().with_timezone(&Shanghai).naive_utc()),
        }
    }

    pub fn get_title(&self) -> String { self.title.clone() }
    pub fn get_content(&self) -> Option<String> { self.content.clone() }
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

impl NewTask {
    pub fn new(
        title: String,
        content: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        NewTask {
            title,
            content,
            created_at,
            updated_at,
        }
    }
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::tasks)]
pub struct PatchTask {
    pub title: String,
    pub content: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl PatchTask {
    pub fn new(
        title: String,
        content: Option<String>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        PatchTask {
            title,
            content,
            updated_at,
        }
    }
}

impl Into<PatchTask> for Task {
    fn into(self) -> PatchTask {
        PatchTask {
            title: self.title,
            content: self.content,
            updated_at: self.updated_at,
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
        }
    }
}
mod test {
    #[test]
    fn test_task_new() {
        let task = super::Task::new(1, "title".to_string(), "content".to_string().into());
        println!("{task}");

        let fixed_dt = crate::utils::time::get_e8_time();
        println!("{:?}", fixed_dt);
    }
}

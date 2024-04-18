use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

use crate::utils::time::get_e8_time;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
    pub user_id: Option<i32>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask {
    pub title: String,
    pub content: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub user_id: Option<i32>,
}

impl Task {
    pub fn new(id: i32, title: String, content: Option<String>, user_id: Option<i32>) -> Self {
        Task {
            id,
            title,
            content,
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
            user_id, /* created_at:
                      * Some(chrono::Local::now().with_timezone(&Shanghai).naive_utc()),
                      * updated_at:
                      * Some(chrono::Local::now().with_timezone(&Shanghai).naive_utc()), */
        }
    }
    pub fn new_empty() -> Self {
        Task {
            id: -1,
            title: String::new(),
            content: String::new().into(),
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
            user_id: Some(-1),
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_content(&self) -> Option<String> {
        self.content.clone()
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
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::tasks)]
pub struct PatchTask {
    pub title: String,
    pub content: Option<String>,
    pub user_id: Option<i32>,
}

impl PatchTask {
    pub fn new(title: String, content: Option<String>, user_id: Option<i32>) -> Self {
        PatchTask {
            title,
            content,
            user_id,
        }
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
#[diesel(table_name = crate::schema::tasks)]
pub struct PutTask {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub user_id: Option<i32>,
}

impl PutTask {
    pub fn new(
        id: i32,
        title: String,
        content: Option<String>,
        updated_at: Option<chrono::NaiveDateTime>,
        user_id: Option<i32>,
    ) -> Self {
        PutTask {
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
        let task = super::Task::new(
            1,
            "title".to_string(),
            "content".to_string().into(),
            Some(4),
        );
        println!("{task}");

        let fixed_dt = crate::utils::time::get_e8_time();
        println!("{:?}", fixed_dt);
    }
}

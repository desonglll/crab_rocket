use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TaskFilter {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at_min: Option<NaiveDateTime>,
    pub created_at_max: Option<NaiveDateTime>,
    pub updated_at_min: Option<NaiveDateTime>,
    pub updated_at_max: Option<NaiveDateTime>,
    pub user_id: Option<i32>,
}

use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PostFilter {
    pub post_id: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub user_id: Option<i32>,
    pub status: Option<String>,
    pub created_at_min: Option<NaiveDateTime>,
    pub created_at_max: Option<NaiveDateTime>,
    pub updated_at_min: Option<NaiveDateTime>,
    pub updated_at_max: Option<NaiveDateTime>,
    pub username: Option<String>,
}

use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserFilter {
    pub username: Option<String>,
    pub role_id: Option<i32>,
    pub created_at_min: Option<NaiveDateTime>,
    pub created_at_max: Option<NaiveDateTime>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub updated_at_min: Option<NaiveDateTime>,
    pub updated_at_max: Option<NaiveDateTime>,
    pub mobile_phone: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

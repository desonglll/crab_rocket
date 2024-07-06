use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RoleFilter {
    pub role_id: Option<i32>,
    pub role_name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<String>,
    pub created_at_min: Option<NaiveDateTime>,
    pub created_at_max: Option<NaiveDateTime>,
    pub updated_at_min: Option<NaiveDateTime>,
    pub updated_at_max: Option<NaiveDateTime>,
}

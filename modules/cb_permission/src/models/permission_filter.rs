use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PermissionFilter {
    pub permission_id: Option<i32>,
    pub permission_name: Option<String>,
    pub permission_description: Option<String>,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub is_active: Option<bool>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at_min: Option<NaiveDateTime>,
    pub created_at_max: Option<NaiveDateTime>,
    pub updated_at_min: Option<NaiveDateTime>,
    pub updated_at_max: Option<NaiveDateTime>,
}

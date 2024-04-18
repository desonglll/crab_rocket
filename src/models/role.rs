use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::time::get_e8_time;
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::role_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub role_id: i32,
    pub role_name: String,
    pub description: Option<String>,
    pub permissions: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
impl Role {
    pub fn new_empty() -> Self {
        Role {
            role_id: -1,
            role_name: String::from("none"),
            description: Some(String::from("none")),
            permissions: Some(String::from("none")),
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::role_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRole {
    pub role_name: String,
    pub description: Option<String>,
    pub permissions: Option<String>,
}

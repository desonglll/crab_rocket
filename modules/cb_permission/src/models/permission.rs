use crab_rocket_utils::time::get_e8_time;
use diesel::{Insertable, Queryable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::permission_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Permission {
    pub permission_id: i32,
    pub permission_name: String,
    pub permission_description: Option<String>,
    pub resource: String,
    pub action: String,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::permission_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostPermission {
    pub permission_name: String,
    pub permission_description: Option<String>,
    pub resource: String,
    pub action: String,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::permission_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchPermission {
    pub permission_name: String,
    pub permission_description: Option<String>,
    pub resource: String,
    pub action: String,
    pub is_active: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub notes: Option<String>,
}

impl Permission {
    pub fn new(
        permission_id: i32,
        permission_name: String,
        permission_description: Option<String>,
        resource: String,
        action: String,
        is_active: Option<bool>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
        created_by: Option<String>,
        updated_by: Option<String>,
        notes: Option<String>,
    ) -> Self {
        Self {
            permission_id,
            permission_name,
            permission_description,
            resource,
            action,
            is_active,
            created_at,
            updated_at,
            created_by,
            updated_by,
            notes,
        }
    }
}

///
/// Demo
// ```
// {
//    "permission_name": "read_articles",
//    "resource": "articles",
//    "action": "read",
//    "is_active": true
// }
// ```
impl PostPermission {
    pub fn new(
        permission_name: String,
        permission_description: Option<String>,
        resource: String,
        action: String,
        is_active: Option<bool>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
        created_by: Option<String>,
        updated_by: Option<String>,
        notes: Option<String>,
    ) -> Self {
        Self {
            permission_name,
            permission_description,
            resource,
            action,
            is_active,
            created_at,
            updated_at,
            created_by,
            updated_by,
            notes,
        }
    }
    pub fn demo() -> Self {
        Self {
            permission_name: String::from("demo"),
            permission_description: Some(String::from("demo")),
            resource: String::from("hello"),
            action: String::from("View"),
            is_active: Some(true),
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
            created_by: Some(String::from("mike")),
            updated_by: Some(String::from("mike")),
            notes: Some(String::from("value")),
        }
    }
}
impl PatchPermission {
    pub fn new(
        permission_name: String,
        permission_description: Option<String>,
        resource: String,
        action: String,
        is_active: Option<bool>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
        created_by: Option<String>,
        updated_by: Option<String>,
        notes: Option<String>,
    ) -> Self {
        Self {
            permission_name,
            permission_description,
            resource,
            action,
            is_active,
            created_at,
            updated_at,
            created_by,
            updated_by,
            notes,
        }
    }
}

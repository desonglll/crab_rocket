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

    pub fn permission_id(&self) -> i32 {
        self.permission_id
    }

    pub fn permission_name(&self) -> &str {
        &self.permission_name
    }

    pub fn permission_description(&self) -> &Option<String> {
        &self.permission_description
    }

    pub fn resource(&self) -> &str {
        &self.resource
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn is_active(&self) -> Option<bool> {
        self.is_active
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn created_by(&self) -> &Option<String> {
        &self.created_by
    }

    pub fn updated_by(&self) -> &Option<String> {
        &self.updated_by
    }

    pub fn notes(&self) -> &Option<String> {
        &self.notes
    }

    pub fn set_permission_id(&mut self, permission_id: i32) {
        self.permission_id = permission_id;
    }

    pub fn set_permission_name(&mut self, permission_name: String) {
        self.permission_name = permission_name;
    }

    pub fn set_permission_description(&mut self, permission_description: Option<String>) {
        self.permission_description = permission_description;
    }

    pub fn set_resource(&mut self, resource: String) {
        self.resource = resource;
    }

    pub fn set_action(&mut self, action: String) {
        self.action = action;
    }

    pub fn set_is_active(&mut self, is_active: Option<bool>) {
        self.is_active = is_active;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }

    pub fn set_created_by(&mut self, created_by: Option<String>) {
        self.created_by = created_by;
    }

    pub fn set_updated_by(&mut self, updated_by: Option<String>) {
        self.updated_by = updated_by;
    }

    pub fn set_notes(&mut self, notes: Option<String>) {
        self.notes = notes;
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

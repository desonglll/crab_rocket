use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::role_table)]
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
    pub fn new(
        role_id: i32,
        role_name: String,
        description: Option<String>,
        permissions: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            role_id,
            role_name,
            description,
            permissions,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::role_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostRole {
    pub role_name: String,
    pub description: Option<String>,
    pub permissions: Option<String>,
}

impl PostRole {
    pub fn new(
        role_name: String,
        description: Option<String>,
        permissions: Option<String>,
    ) -> Self {
        Self {
            role_name,
            description,
            permissions,
        }
    }

    pub fn demo() -> Self {
        Self {
            role_name: String::from("DemoUser"),
            description: Some(String::from("Administrator role with full access")),
            permissions: Some(String::from("admin:full_access,user:view,post:edit")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::role_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchRole {
    pub role_name: String,
    pub description: Option<String>,
    pub permissions: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl PatchRole {
    pub fn new(
        role_name: String,
        description: Option<String>,
        permissions: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            role_name,
            description,
            permissions,
            created_at,
            updated_at,
        }
    }
}

impl From<Role> for PostRole {
    fn from(role: Role) -> Self {
        PostRole {
            role_name: role.role_name,
            description: role.description,
            permissions: role.permissions,
        }
    }
}

impl From<Role> for PatchRole {
    fn from(role: Role) -> Self {
        PatchRole {
            role_name: role.role_name,
            description: role.description,
            permissions: role.permissions,
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

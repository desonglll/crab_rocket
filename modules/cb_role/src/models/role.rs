use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::role_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    role_id: i32,
    role_name: String,
    description: Option<String>,
    permissions: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
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

    pub fn role_id(&self) -> i32 {
        self.role_id
    }

    pub fn role_name(&self) -> &str {
        &self.role_name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn permissions(&self) -> &Option<String> {
        &self.permissions
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn set_role_id(&mut self, role_id: i32) {
        self.role_id = role_id;
    }

    pub fn set_role_name(&mut self, role_name: String) {
        self.role_name = role_name;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_permissions(&mut self, permissions: Option<String>) {
        self.permissions = permissions;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::role_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostRole {
    role_name: String,
    description: Option<String>,
    permissions: Option<String>,
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
            role_name: String::from("admin"),
            description: Some(String::from("Administrator role with full access")),
            permissions: Some(String::from("admin:full_access,user:view,post:edit")),
        }
    }

    pub fn role_name(&self) -> &str {
        &self.role_name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn permissions(&self) -> &Option<String> {
        &self.permissions
    }

    pub fn set_role_name(&mut self, role_name: String) {
        self.role_name = role_name;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_permissions(&mut self, permissions: Option<String>) {
        self.permissions = permissions;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::role_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchRole {
    role_name: String,
    description: Option<String>,
    permissions: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
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

    pub fn role_name(&self) -> &str {
        &self.role_name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn permissions(&self) -> &Option<String> {
        &self.permissions
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn set_role_name(&mut self, role_name: String) {
        self.role_name = role_name;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_permissions(&mut self, permissions: Option<String>) {
        self.permissions = permissions;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
}

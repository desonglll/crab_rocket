// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = crab_rocket_schema::schema::category_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    category_id: i32,
    name: String,
    description: Option<String>,
    parent_id: Option<i32>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}

impl Category {
    pub fn new(
        category_id: i32,
        name: String,
        description: Option<String>,
        parent_id: Option<i32>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            category_id,
            name,
            description,
            parent_id,
            created_at,
            updated_at,
        }
    }

    pub fn category_id(&self) -> i32 {
        self.category_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn parent_id(&self) -> Option<i32> {
        self.parent_id
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn set_category_id(&mut self, category_id: i32) {
        self.category_id = category_id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_parent_id(&mut self, parent_id: Option<i32>) {
        self.parent_id = parent_id;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::category_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostCategory {
    name: String,
    description: Option<String>,
    parent_id: Option<i32>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}

impl PostCategory {
    pub fn new(
        name: String,
        description: Option<String>,
        parent_id: Option<i32>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            name,
            description,
            parent_id,
            created_at,
            updated_at,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn parent_id(&self) -> Option<i32> {
        self.parent_id
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_parent_id(&mut self, parent_id: Option<i32>) {
        self.parent_id = parent_id;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::category_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchCategory {
    name: String,
    description: Option<String>,
    parent_id: Option<i32>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}

impl PatchCategory {
    pub fn new(
        name: String,
        description: Option<String>,
        parent_id: Option<i32>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            name,
            description,
            parent_id,
            created_at,
            updated_at,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn parent_id(&self) -> Option<i32> {
        self.parent_id
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_parent_id(&mut self, parent_id: Option<i32>) {
        self.parent_id = parent_id;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
}

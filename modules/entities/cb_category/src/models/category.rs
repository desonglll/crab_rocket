use crab_rocket_utils::time::get_e8_time;
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
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::category_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostCategory {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
impl PostCategory {
    pub fn demo() -> Self {
        Self {
            name: "post_category".to_owned(),
            description: None,
            parent_id: None,
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
        }
    }
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
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::category_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchCategory {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
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
}

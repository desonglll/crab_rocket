// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crab_rocket_utils::time::get_e8_time;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = crab_rocket_schema::schema::supplier_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Supplier {
    pub supplier_id: i32,
    pub name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Supplier {
    pub fn new(
        supplier_id: i32,
        name: String,
        address: Option<String>,
        phone_number: Option<String>,
        email: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            supplier_id,
            name,
            address,
            phone_number,
            email,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::supplier_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostSupplier {
    pub name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl PostSupplier {
    pub fn new(
        name: String,
        address: Option<String>,
        phone_number: Option<String>,
        email: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            name,
            address,
            phone_number,
            email,
            created_at,
            updated_at,
        }
    }
    pub fn demo() -> Self {
        Self {
            name: "Demo Supplier".to_string(),
            address: Some("123 Demo St, Demo City".to_string()),
            phone_number: Some("123-456-7890".to_string()),
            email: Some("demo@example.com".to_string()),
            created_at: Some(get_e8_time()), // 通常由数据库生成
            updated_at: Some(get_e8_time()), // 通常由数据库生成
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::supplier_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchSupplier {
    pub name: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl PatchSupplier {
    pub fn new(
        name: String,
        address: Option<String>,
        phone_number: Option<String>,
        email: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            name,
            address,
            phone_number,
            email,
            created_at,
            updated_at,
        }
    }
}

impl From<Supplier> for PatchSupplier {
    fn from(supplier: Supplier) -> Self {
        PatchSupplier {
            name: supplier.name,
            address: supplier.address,
            phone_number: supplier.phone_number,
            email: supplier.email,
            created_at: supplier.created_at,
            updated_at: supplier.updated_at,
        }
    }
}

impl From<Supplier> for PostSupplier {
    fn from(supplier: Supplier) -> Self {
        PostSupplier {
            name: supplier.name,
            address: supplier.address,
            phone_number: supplier.phone_number,
            email: supplier.email,
            created_at: supplier.created_at,
            updated_at: supplier.updated_at,
        }
    }
}

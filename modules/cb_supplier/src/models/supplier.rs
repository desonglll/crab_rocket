use crab_rocket_utils::time::get_e8_time;
// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = crab_rocket_schema::schema::supplier_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Supplier {
    supplier_id: i32,
    name: String,
    address: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
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

    pub fn supplier_id(&self) -> i32 {
        self.supplier_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> &Option<String> {
        &self.address
    }

    pub fn phone_number(&self) -> &Option<String> {
        &self.phone_number
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }

    pub fn set_supplier_id(&mut self, supplier_id: i32) {
        self.supplier_id = supplier_id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }

    pub fn set_phone_number(&mut self, phone_number: Option<String>) {
        self.phone_number = phone_number;
    }

    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
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
#[diesel(table_name = crab_rocket_schema::schema::supplier_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostSupplier {
    name: String,
    address: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
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
    name: String,
    address: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn address(&self) -> &Option<String> {
        &self.address
    }

    pub fn phone_number(&self) -> &Option<String> {
        &self.phone_number
    }

    pub fn email(&self) -> &Option<String> {
        &self.email
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

    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }

    pub fn set_phone_number(&mut self, phone_number: Option<String>) {
        self.phone_number = phone_number;
    }

    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
}

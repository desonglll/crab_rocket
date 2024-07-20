use chrono::NaiveDateTime;
// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = crab_rocket_schema::schema::product_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub product_id: i32,
    pub user_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub image: Option<String>,
    pub price: Option<f64>,
    pub discount_price: Option<f64>,
    pub is_discounted: Option<bool>,
    pub is_valid: Option<bool>,
    pub inventory: Option<i32>,
    pub is_in_stock: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub supplier_id: Option<i32>,
    pub weight: Option<f64>,
    pub dimensions: Option<String>,
    pub status: Option<String>,
    pub public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = crab_rocket_schema::schema::product_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostProduct {
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub image: Option<String>,
    pub price: Option<f64>,
    pub discount_price: Option<f64>,
    pub is_discounted: Option<bool>,
    pub is_valid: Option<bool>,
    pub inventory: Option<i32>,
    pub is_in_stock: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub supplier_id: Option<i32>,
    pub weight: Option<f64>,
    pub dimensions: Option<String>,
    pub status: Option<String>,
    pub public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable, QueryableByName)]
#[diesel(table_name = crab_rocket_schema::schema::product_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchProduct {
    pub user_id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub image: Option<String>,
    pub price: Option<f64>,
    pub discount_price: Option<f64>,
    pub is_discounted: Option<bool>,
    pub is_valid: Option<bool>,
    pub inventory: Option<i32>,
    pub is_in_stock: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub supplier_id: Option<i32>,
    pub weight: Option<f64>,
    pub dimensions: Option<String>,
    pub status: Option<String>,
    pub public: Option<bool>,
}

use chrono::NaiveDateTime;
// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crab_rocket_utils::time::get_e8_time;

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

impl PostProduct {
    pub fn demo() -> Self {
        Self {
            user_id: Some(1),
            name: "Demo Product".to_string(),
            description: Some("This is a demo product.".to_string()),
            sku: "DEMO123".to_string(),
            image: Some("http://example.com/demo.jpg".to_string()),
            price: Some(19.99),
            discount_price: Some(14.99),
            is_discounted: Some(true),
            is_valid: Some(true),
            inventory: Some(100),
            is_in_stock: Some(true),
            created_at: Some(get_e8_time()),
            updated_at: Some(get_e8_time()),
            supplier_id: Some(42),
            weight: Some(1.5),
            dimensions: Some("10x10x10 cm".to_string()),
            status: Some("Available".to_string()),
            public: Some(true),
        }
    }
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

impl From<Product> for PostProduct {
    fn from(product: Product) -> Self {
        PostProduct {
            user_id: product.user_id,
            name: product.name,
            description: product.description,
            sku: product.sku,
            image: product.image,
            price: product.price,
            discount_price: product.discount_price,
            is_discounted: product.is_discounted,
            is_valid: product.is_valid,
            inventory: product.inventory,
            is_in_stock: product.is_in_stock,
            created_at: product.created_at,
            updated_at: product.updated_at,
            supplier_id: product.supplier_id,
            weight: product.weight,
            dimensions: product.dimensions,
            status: product.status,
            public: product.public,
        }
    }
}

impl From<Product> for PatchProduct {
    fn from(product: Product) -> Self {
        PatchProduct {
            user_id: product.user_id,
            name: product.name,
            description: product.description,
            sku: product.sku,
            image: product.image,
            price: product.price,
            discount_price: product.discount_price,
            is_discounted: product.is_discounted,
            is_valid: product.is_valid,
            inventory: product.inventory,
            is_in_stock: product.is_in_stock,
            created_at: product.created_at,
            updated_at: product.updated_at,
            supplier_id: product.supplier_id,
            weight: product.weight,
            dimensions: product.dimensions,
            status: product.status,
            public: product.public,
        }
    }
}

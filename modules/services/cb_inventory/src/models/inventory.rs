use chrono::NaiveDateTime;
use crab_rocket_utils::time::get_e8_time;
// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::inventory_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Inventory {
    pub inventory_id: i32,
    pub product_id: Option<i32>,
    pub location: Option<String>,
    pub quantity: Option<i32>,
    pub last_updated: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::inventory_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostInventory {
    pub product_id: Option<i32>,
    pub location: Option<String>,
    pub quantity: Option<i32>,
    pub last_updated: Option<NaiveDateTime>,
}
impl PostInventory {
    pub fn demo() -> Self {
        Self {
            product_id: Some(1),
            location: None,
            quantity: None,
            last_updated: Some(get_e8_time()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::inventory_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchInventory {
    pub product_id: Option<i32>,
    pub location: Option<String>,
    pub quantity: Option<i32>,
    pub last_updated: Option<NaiveDateTime>,
}

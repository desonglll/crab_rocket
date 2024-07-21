use chrono::NaiveDateTime;
// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::order_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub order_id: i32,
    pub customer_id: Option<i32>,
    pub order_date: Option<NaiveDateTime>,
    pub total_amount: Option<f64>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::order_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostOrder {
    pub customer_id: Option<i32>,
    pub order_date: Option<NaiveDateTime>,
    pub total_amount: Option<f64>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::order_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchOrder {
    pub customer_id: Option<i32>,
    pub order_date: Option<NaiveDateTime>,
    pub total_amount: Option<f64>,
    pub status: Option<String>,
}

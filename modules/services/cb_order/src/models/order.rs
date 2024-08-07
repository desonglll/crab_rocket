use chrono::NaiveDateTime;
// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crab_rocket_utils::time::get_e8_time;

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

impl PostOrder {
    pub fn demo() -> Self {
        Self {
            customer_id: None,

            order_date: Some(get_e8_time()),
            total_amount: None,
            status: None,
        }
    }
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

impl From<Order> for PostOrder {
    fn from(order: Order) -> Self {
        PostOrder {
            customer_id: order.customer_id,
            order_date: order.order_date,
            total_amount: order.total_amount,
            status: order.status,
        }
    }
}

impl From<Order> for PatchOrder {
    fn from(order: Order) -> Self {
        PatchOrder {
            customer_id: order.customer_id,
            order_date: order.order_date,
            total_amount: order.total_amount,
            status: order.status,
        }
    }
}

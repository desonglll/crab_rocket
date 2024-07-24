use chrono::NaiveDateTime;
// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::shipment_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Shipment {
    pub shipment_id: i32,
    pub order_id: Option<i32>,
    pub shipment_date: Option<NaiveDateTime>,
    pub delivery_address: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::shipment_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostShipment {
    pub order_id: Option<i32>,
    pub shipment_date: Option<NaiveDateTime>,
    pub delivery_address: Option<String>,
    pub status: Option<String>,
}

impl PostShipment {
    pub fn demo() -> Self {
        Self {
            order_id: None,
            shipment_date: None,
            delivery_address: None,
            status: None,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::shipment_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchShipment {
    pub order_id: Option<i32>,
    pub shipment_date: Option<NaiveDateTime>,
    pub delivery_address: Option<String>,
    pub status: Option<String>,
}

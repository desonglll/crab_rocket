// 定义供应商结构体
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::customer_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Customer {
    pub customer_id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::customer_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostCustomer {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}

impl PostCustomer {
    pub fn demo() -> Self {
        Self {
            name: "Customer".to_owned(),
            email: "aaa@example.com".to_owned(),
            phone: None,
            address: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::customer_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchCustomer {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
}

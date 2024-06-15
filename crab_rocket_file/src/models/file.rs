use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid; // 添加这一行

#[derive(Insertable, Serialize, Deserialize, Selectable, Debug, Queryable, Default)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::file_table)]
pub struct File {
    pub id: Uuid,
    pub file_name: String,
    pub file_url: String,
    pub uploaded_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Selectable, Debug, Queryable, Default)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crab_rocket_schema::schema::file_table)]
pub struct NewFile<'a> {
    pub id: Uuid,
    pub file_name: &'a str,
    pub file_url: &'a str,
}

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use rocket::serde::uuid::Uuid;
use rocket::serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct File {
    pub id: Uuid,
    pub file_name: String,
    pub file_url: String,
    pub uploaded_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::files)]
pub struct NewFile<'a> {
    pub id: Uuid,
    pub file_name: &'a str,
    pub file_url: &'a str,
}

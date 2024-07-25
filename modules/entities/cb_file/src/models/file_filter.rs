use chrono::NaiveDateTime;
use rocket::serde::Deserialize;
use uuid::Uuid; // 确保你有这个依赖

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct FileFilter {
    pub file_id: Option<Uuid>,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub uploaded_at_min: Option<NaiveDateTime>,
    pub uploaded_at_max: Option<NaiveDateTime>,
}

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostInfo {
    pub count: i64,
}

impl PostInfo {
    pub fn new_empty() -> Self { PostInfo { count: -1 } }
}

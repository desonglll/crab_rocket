use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostParam {
    pub user_id: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

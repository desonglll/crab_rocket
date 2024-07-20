use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FollowParam {
    pub following_user_id: Option<i32>,
    pub followed_user_id: Option<i32>,
}

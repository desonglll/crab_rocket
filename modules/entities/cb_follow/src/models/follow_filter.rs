use chrono::NaiveDateTime;
use rocket::serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct FollowFilter {
    pub following_user_id: Option<i32>,
    pub followed_user_id: Option<i32>,
    pub created_at_min: Option<NaiveDateTime>,
    pub created_at_max: Option<NaiveDateTime>,
    pub follow_id: Option<i32>,
}

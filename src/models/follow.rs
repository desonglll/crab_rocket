use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::utils::time::get_e8_time;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::follows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Follow {
    following_user_id: i32,
    followed_user_id: i32,
    created_at: Option<chrono::NaiveDateTime>,
    follow_id: i32,
}

impl Follow {
    pub fn new(follow_id: i32, following_user_id: i32, followed_user_id: i32) -> Self {
        Follow {
            follow_id,
            followed_user_id,
            following_user_id,
            created_at: Some(get_e8_time()),
        }
    }
    pub fn new_empty() -> Self {
        Follow {
            follow_id: -1,
            followed_user_id: 0,
            following_user_id: 0,
            created_at: Some(get_e8_time()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::follows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewFollow {
    pub following_user_id: i32,
    pub followed_user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl NewFollow {
    pub fn new(following_user_id: i32, followed_user_id: i32) -> Self {
        NewFollow {
            followed_user_id,
            following_user_id,
            created_at: Some(get_e8_time()),
        }
    }
}

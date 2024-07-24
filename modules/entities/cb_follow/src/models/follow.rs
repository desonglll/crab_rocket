use crab_rocket_utils::time::get_e8_time;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::follow_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Follow {
    pub following_user_id: i32,
    pub followed_user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub follow_id: i32,
}

impl Follow {
    pub fn new(
        following_user_id: i32,
        followed_user_id: i32,
        created_at: Option<chrono::NaiveDateTime>,
        follow_id: i32,
    ) -> Self {
        Self {
            following_user_id,
            followed_user_id,
            created_at,
            follow_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::follow_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostFollow {
    pub following_user_id: i32,
    pub followed_user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl PostFollow {
    pub fn new(
        following_user_id: i32,
        followed_user_id: i32,
        // created_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            following_user_id,
            followed_user_id,
            created_at: Some(get_e8_time()),
        }
    }

    pub fn demo() -> Self {
        Self {
            followed_user_id: 4,
            following_user_id: 1,
            created_at: Some(get_e8_time()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::follow_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchFollow {
    pub following_user_id: i32,
    pub followed_user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl PatchFollow {
    pub fn new(
        following_user_id: i32,
        followed_user_id: i32,
        created_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            following_user_id,
            followed_user_id,
            created_at,
        }
    }
}
impl From<Follow> for PostFollow {
    fn from(follow: Follow) -> Self {
        PostFollow {
            following_user_id: follow.following_user_id,
            followed_user_id: follow.followed_user_id,
            created_at: follow.created_at,
        }
    }
}

impl From<Follow> for PatchFollow {
    fn from(follow: Follow) -> Self {
        PatchFollow {
            following_user_id: follow.following_user_id,
            followed_user_id: follow.followed_user_id,
            created_at: follow.created_at,
        }
    }
}

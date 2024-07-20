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
    following_user_id: i32,
    followed_user_id: i32,
    created_at: Option<chrono::NaiveDateTime>,
    follow_id: i32,
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

    pub fn following_user_id(&self) -> i32 {
        self.following_user_id
    }

    pub fn followed_user_id(&self) -> i32 {
        self.followed_user_id
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn follow_id(&self) -> i32 {
        self.follow_id
    }

    pub fn set_following_user_id(&mut self, following_user_id: i32) {
        self.following_user_id = following_user_id;
    }

    pub fn set_followed_user_id(&mut self, followed_user_id: i32) {
        self.followed_user_id = followed_user_id;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn set_follow_id(&mut self, follow_id: i32) {
        self.follow_id = follow_id;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::follow_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostFollow {
    following_user_id: i32,
    followed_user_id: i32,
    created_at: Option<chrono::NaiveDateTime>,
}

impl PostFollow {
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

    pub fn demo() -> Self {
        Self {
            followed_user_id: 3,
            following_user_id: 1,
            created_at: Some(get_e8_time()),
        }
    }
    pub fn following_user_id(&self) -> i32 {
        self.following_user_id
    }

    pub fn followed_user_id(&self) -> i32 {
        self.followed_user_id
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn set_following_user_id(&mut self, following_user_id: i32) {
        self.following_user_id = following_user_id;
    }

    pub fn set_followed_user_id(&mut self, followed_user_id: i32) {
        self.followed_user_id = followed_user_id;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::follow_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchFollow {
    following_user_id: i32,
    followed_user_id: i32,
    created_at: Option<chrono::NaiveDateTime>,
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

    pub fn following_user_id(&self) -> i32 {
        self.following_user_id
    }

    pub fn followed_user_id(&self) -> i32 {
        self.followed_user_id
    }

    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }

    pub fn set_following_user_id(&mut self, following_user_id: i32) {
        self.following_user_id = following_user_id;
    }

    pub fn set_followed_user_id(&mut self, followed_user_id: i32) {
        self.followed_user_id = followed_user_id;
    }

    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }
}

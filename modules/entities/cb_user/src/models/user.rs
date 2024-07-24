use std::fmt::Display;

use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crab_rocket_utils::time::get_e8_time;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::user_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub role_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub password: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub mobile_phone: String,
}

impl User {
    pub fn new(
        user_id: i32,
        username: String,
        role_id: Option<i32>,
        created_at: Option<chrono::NaiveDateTime>,
        email: Option<String>,
        password: String,
        full_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        updated_at: Option<chrono::NaiveDateTime>,
        mobile_phone: String,
    ) -> Self {
        Self {
            user_id,
            username,
            role_id,
            created_at,
            email,
            password,
            full_name,
            avatar_url,
            bio,
            updated_at,
            mobile_phone,
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User ID: {}\nUsername: {}\nRole: {:?}\nCreated At: {:?}\nEmail: {:?}\nFullname: {:?}\nAvatar URL: {:?}\nBio: {:?}\nUpdated At: {:?}\nMobile Phone: {}",
            self.user_id,
            self.username,
            self.role_id.unwrap(),
            self.created_at.unwrap(),
            self.email,
            self.full_name.clone().unwrap(),
            self.avatar_url.clone().unwrap(),
            self.bio.clone().unwrap(),
            self.updated_at.unwrap(),
            self.mobile_phone
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable, Default)]
#[diesel(table_name = crab_rocket_schema::schema::user_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostUser {
    pub username: String,
    pub role_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub password: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub mobile_phone: String,
}

impl PostUser {
    pub fn new(
        username: String,
        role_id: Option<i32>,
        created_at: Option<chrono::NaiveDateTime>,
        email: Option<String>,
        password: String,
        full_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        updated_at: Option<chrono::NaiveDateTime>,
        mobile_phone: String,
    ) -> Self {
        Self {
            username,
            role_id,
            created_at,
            email,
            password,
            full_name,
            avatar_url,
            bio,
            updated_at,
            mobile_phone,
        }
    }

    pub fn demo() -> Self {
        Self {
            username: "username".to_string(),
            role_id: Some(1),
            created_at: Some(get_e8_time()),
            email: Some(String::from("email")),
            password: "password".to_string(),
            full_name: Some(String::from("fullname")),
            avatar_url: Some(String::from("avatar_url")),
            bio: Some(String::from("bio")),
            updated_at: Some(get_e8_time()),
            mobile_phone: "mobile_phone".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crab_rocket_schema::schema::user_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchUser {
    pub username: String,
    pub role_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub password: String,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub mobile_phone: String,
}

impl PatchUser {
    pub fn new(
        username: String,
        role_id: Option<i32>,
        created_at: Option<chrono::NaiveDateTime>,
        email: Option<String>,
        password: String,
        full_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        updated_at: Option<chrono::NaiveDateTime>,
        mobile_phone: String,
    ) -> Self {
        Self {
            username,
            role_id,
            created_at,
            email,
            password,
            full_name,
            avatar_url,
            bio,
            updated_at,
            mobile_phone,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_user_new() {
        use super::User;
        use crab_rocket_utils::time::get_e8_time;
        let user = User::new(
            1,
            "john_doe".to_string(),
            Some(1),
            Some(get_e8_time()),
            Some("john.doe@example.com".to_string()),
            "password_hash".to_string(),
            Some("John Doe".to_string()),
            Some("https://example.com/avatar.jpg".to_string()),
            Some("Software engineer".to_string()),
            Some(get_e8_time()),
            "1234567890".to_string(),
        );
        println!("{user:?}");
    }
}

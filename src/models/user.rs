use std::fmt::Display;

use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    user_id: i32,
    username: String,
    role: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    email: Option<String>,
    password: String,
    fullname: Option<String>,
    avatar_url: Option<String>,
    bio: Option<String>,
    updated_at: Option<chrono::NaiveDateTime>,
    mobile_phone: String,
}

impl User {
    fn new(
        user_id: i32,
        username: String,
        role: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        email: Option<String>,
        password: String,
        fullname: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        updated_at: Option<chrono::NaiveDateTime>,
        mobile_phone: String,
    ) -> Self {
        User {
            user_id,
            username,
            role,
            created_at,
            email,
            password,
            fullname,
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
            self.role,
            self.created_at,
            self.email,
            self.fullname,
            self.avatar_url,
            self.bio,
            self.updated_at,
            self.mobile_phone
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    username: String,
    role: Option<String>,
    created_at: Option<chrono::NaiveDateTime>,
    email: Option<String>,
    password: String,
    fullname: Option<String>,
    avatar_url: Option<String>,
    bio: Option<String>,
    updated_at: Option<chrono::NaiveDateTime>,
    mobile_phone: String,
}

impl NewUser {
    fn new(
        username: String,
        role: Option<String>,
        created_at: Option<chrono::NaiveDateTime>,
        email: Option<String>,
        password: String,
        fullname: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        updated_at: Option<chrono::NaiveDateTime>,
        mobile_phone: String,
    ) -> Self {
        NewUser {
            username,
            role,
            created_at,
            email,
            password,
            fullname,
            avatar_url,
            bio,
            updated_at,
            mobile_phone,
        }
    }
}

mod test {
    use super::User;
    use crate::utils::time::get_e8_time;

    #[test]
    fn test_user_new() {
        let user = User::new(
            1,
            "john_doe".to_string(),
            Some("admin".to_string()),
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

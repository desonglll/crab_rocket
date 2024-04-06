use std::fmt::Display;

use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use rocket::serde::{Deserialize, Serialize};

use crate::utils::time::get_e8_time;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub role: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub password: String,
    pub fullname: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub mobile_phone: String,
}

impl User {
    pub fn new(
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

    pub fn new_empty() -> Self {
        User {
            user_id: -1,
            username: String::from(""),
            role: Some(String::from("")),
            created_at: Some(get_e8_time()),
            email: Some(String::from("")),
            password: String::from(""),
            fullname: Some(String::from("")),
            avatar_url: Some(String::from("")),
            bio: Some(String::from("")),
            updated_at: Some(get_e8_time()),
            mobile_phone: String::from(""),
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
            self.role.clone().unwrap(),
            self.created_at.unwrap(),
            self.email,
            self.fullname.clone().unwrap(),
            self.avatar_url.clone().unwrap(),
            self.bio.clone().unwrap(),
            self.updated_at.unwrap(),
            self.mobile_phone
        )
    }
}
impl Into<PatchUser> for User {
    fn into(self) -> PatchUser {
        PatchUser {
            username: self.username,
            role: self.role,
            created_at: self.created_at,
            email: self.email,
            password: self.password,
            fullname: self.fullname,
            avatar_url: self.avatar_url,
            bio: self.bio,
            updated_at: self.updated_at,
            mobile_phone: self.mobile_phone,
        }
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
    pub fn new(
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

impl Into<PatchUser> for NewUser {
    fn into(self) -> PatchUser {
        PatchUser {
            username: self.username,
            role: self.role,
            created_at: self.created_at,
            email: self.email,
            password: self.password,
            fullname: self.fullname,
            avatar_url: self.avatar_url,
            bio: self.bio,
            updated_at: self.updated_at,
            mobile_phone: self.mobile_phone,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchUser {
    pub username: String,
    pub role: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub email: Option<String>,
    pub password: String,
    pub fullname: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub mobile_phone: String,
}

impl PatchUser {
    pub fn new(
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
        PatchUser {
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

    pub fn new_empty() -> Self {
        PatchUser {
            username: String::from(""),
            role: Some(String::from("")),
            created_at: Some(get_e8_time()),
            email: Some(String::from("")),
            password: String::from(""),
            fullname: Some(String::from("")),
            avatar_url: Some(String::from("")),
            bio: Some(String::from("")),
            updated_at: Some(get_e8_time()),
            mobile_phone: String::from(""),
        }
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn test_user_new() {
        use super::User;
        use crate::utils::time::get_e8_time;
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

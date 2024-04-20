use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};
use rocket::serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Default)]
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
        Self {
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
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn role(&self) -> &Option<String> {
        &self.role
    }
    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }
    pub fn email(&self) -> &Option<String> {
        &self.email
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn fullname(&self) -> &Option<String> {
        &self.fullname
    }
    pub fn avatar_url(&self) -> &Option<String> {
        &self.avatar_url
    }
    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }
    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }
    pub fn mobile_phone(&self) -> &str {
        &self.mobile_phone
    }
    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_role(&mut self, role: Option<String>) {
        self.role = role;
    }
    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }
    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    pub fn set_fullname(&mut self, fullname: Option<String>) {
        self.fullname = fullname;
    }
    pub fn set_avatar_url(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
    }
    pub fn set_bio(&mut self, bio: Option<String>) {
        self.bio = bio;
    }
    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
    pub fn set_mobile_phone(&mut self, mobile_phone: String) {
        self.mobile_phone = mobile_phone;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable, Default)]
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
        Self {
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
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn role(&self) -> &Option<String> {
        &self.role
    }
    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }
    pub fn email(&self) -> &Option<String> {
        &self.email
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn fullname(&self) -> &Option<String> {
        &self.fullname
    }
    pub fn avatar_url(&self) -> &Option<String> {
        &self.avatar_url
    }
    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }
    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }
    pub fn mobile_phone(&self) -> &str {
        &self.mobile_phone
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_role(&mut self, role: Option<String>) {
        self.role = role;
    }
    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }
    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    pub fn set_fullname(&mut self, fullname: Option<String>) {
        self.fullname = fullname;
    }
    pub fn set_avatar_url(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
    }
    pub fn set_bio(&mut self, bio: Option<String>) {
        self.bio = bio;
    }
    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
    pub fn set_mobile_phone(&mut self, mobile_phone: String) {
        self.mobile_phone = mobile_phone;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PatchUser {
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
        Self {
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
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn role(&self) -> &Option<String> {
        &self.role
    }
    pub fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        self.created_at
    }
    pub fn email(&self) -> &Option<String> {
        &self.email
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn fullname(&self) -> &Option<String> {
        &self.fullname
    }
    pub fn avatar_url(&self) -> &Option<String> {
        &self.avatar_url
    }
    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }
    pub fn updated_at(&self) -> Option<chrono::NaiveDateTime> {
        self.updated_at
    }
    pub fn mobile_phone(&self) -> &str {
        &self.mobile_phone
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_role(&mut self, role: Option<String>) {
        self.role = role;
    }
    pub fn set_created_at(&mut self, created_at: Option<chrono::NaiveDateTime>) {
        self.created_at = created_at;
    }
    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    pub fn set_fullname(&mut self, fullname: Option<String>) {
        self.fullname = fullname;
    }
    pub fn set_avatar_url(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
    }
    pub fn set_bio(&mut self, bio: Option<String>) {
        self.bio = bio;
    }
    pub fn set_updated_at(&mut self, updated_at: Option<chrono::NaiveDateTime>) {
        self.updated_at = updated_at;
    }
    pub fn set_mobile_phone(&mut self, mobile_phone: String) {
        self.mobile_phone = mobile_phone;
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

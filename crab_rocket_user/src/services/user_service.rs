use crate::models::user::{NewUser, PatchUser, User};

pub trait GetUser {
    fn insert_single_user(user: &NewUser) -> Result<User, Box<dyn std::error::Error>>;
    fn get_all_users() -> Result<Vec<User>, Box<dyn std::error::Error>>;
    fn get_user_by_id(id: i32) -> Result<User, Box<dyn std::error::Error>>;
    fn update_user_by_id(id: i32, user: &PatchUser) -> Result<User, Box<dyn std::error::Error>>;
    fn delete_user_by_id(id: i32) -> Result<User, Box<dyn std::error::Error>>;
}

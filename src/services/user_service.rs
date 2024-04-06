use crate::models::user::{NewUser, PatchUser, User};

pub trait GetUser {
    fn insert_single_user(user: &NewUser) -> Result<User, diesel::result::Error>;
    fn get_all_users() -> Result<Vec<User>, diesel::result::Error>;
    fn get_user_by_id(id: i32) -> Result<User, diesel::result::Error>;
    fn update_user_by_id(id: i32, user: &PatchUser) -> Result<User, diesel::result::Error>;
    fn delete_user_by_id(id: i32) -> Result<User, diesel::result::Error>;
}

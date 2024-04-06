use crate::models::user::{NewUser, User};

pub trait GetUser {
    fn insert_single_user(user: &NewUser) -> Result<User, diesel::result::Error>;

    fn get_all_users() -> Result<Vec<User>, diesel::result::Error>;
}

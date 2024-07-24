use rocket::State;

use crab_rocket_schema::DbPool;
use crab_rocket_user::services::user_service::UserService;

use crate::models::log_error::LogError;

pub trait LogTrait {
    fn is_user_exists(pool: &State<DbPool>, username: String) -> Result<bool, LogError> {
        match UserService::is_user_exists(pool, username) {
            true => Ok(true),
            false => Err(LogError::NotFound)
        }
    }
}
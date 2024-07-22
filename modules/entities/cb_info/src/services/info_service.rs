use crab_rocket_schema::DbPool;

use crate::models::info::Info;

pub trait GetInfo {
    fn get_info(pool: &DbPool) -> Result<Info, Box<dyn std::error::Error>>;
}

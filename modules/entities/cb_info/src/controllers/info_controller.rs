use crab_rocket_schema::DbPool;

use crate::{models::info::Info, services::info_service::GetInfo};

pub fn get_info(pool: &DbPool) -> (i32, String, Info) {
    match Info::get_info(pool) {
        Ok(data) => (200, String::from("Ok"), data),
        Err(e) => (204, e.to_string(), Info::default()),
    }
}

use crate::{models::info::Info, services::info_service::GetInfo};

pub fn get_info() -> (i32, String, Info) {
    match Info::get_info() {
        Ok(data) => (200, String::from("Ok"), data),
        Err(e) => (204, e.to_string(), Info::default()),
    }
}

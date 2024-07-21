use crate::models::info::Info;

pub trait GetInfo {
    fn get_info() -> Result<Info, Box<dyn std::error::Error>>;
}

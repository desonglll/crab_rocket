use crate::{mappers::info_mapper::get_info, models::info::Info, services::info_service::GetInfo};
use crab_rocket_schema::establish_pg_connection;

impl GetInfo for Info {
    fn get_info() -> Result<Info, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => {
                match get_info(&mut conn) {
                    Ok(data) => Ok(data),
                    Err(e) => {
                        println!("{e:?}");
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
}

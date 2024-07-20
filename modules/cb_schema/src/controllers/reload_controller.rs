use crate::establish_pg_connection;
use crate::models::reload_count::ReloadCount;
pub fn get_reload_counts_controller() -> (i32, String, Vec<ReloadCount>) {
    match establish_pg_connection() {
        Ok(mut conn) => {
            match crate::mappers::schema_mappers::get_reload_counts(&mut conn) {
                Ok(data) => (200, String::from("GET RELOAD COUNT OK"), data),
                Err(e) => (204, e.to_string(), Vec::new()),
            }
        }
        Err(e) => {
            println!("{e:?}");
            (204, e.to_string(), Vec::new())
        }
    }
}

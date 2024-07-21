use crab_rocket_schema::routes::schema_routes::*;
use rocket::{routes, Route};
pub fn schemas_routes() -> Vec<Route> {
    routes![get_reload_count]
}

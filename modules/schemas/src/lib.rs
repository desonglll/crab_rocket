use rocket::{Route, routes};

use crab_rocket_schema::routes::schema_routes::*;

pub fn schemas_routes() -> Vec<Route> {
    routes![get_reload_count]
}

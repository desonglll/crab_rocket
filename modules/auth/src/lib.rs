use rocket::{Route, routes};

use routes::auth_routes::*;

pub mod mappers {}
pub mod models {
    pub mod log_error;
    pub mod login;
    pub mod logout;
    pub mod session;
}
pub mod routes {
    pub mod auth_routes;
}

pub fn auth_routes() -> Vec<Route> {
    routes![login, logout]
}

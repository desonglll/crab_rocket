pub mod controllers;
pub mod mappers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod services;
pub mod utils;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_pg_connection() -> Result<PgConnection, diesel::result::ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(e),
    }
}

use colored::Colorize;
use diesel::{Connection, PgConnection};
use std::env;
use dotenv::dotenv;

pub mod common;
pub mod controllers;
pub mod mappers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod update_reload;

pub fn establish_pg_connection() -> Result<PgConnection, diesel::result::ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{} {}", "Current DATABASE_URL: \t".green(), database_url.to_string().blue());

    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(e),
    }
}

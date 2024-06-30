use std::env;

use diesel::{Connection, PgConnection};

pub mod common;
pub mod controllers;
pub mod mappers;
pub mod routes;
pub mod schema;
pub mod update_reload;
pub fn establish_pg_connection() -> Result<PgConnection, diesel::result::ConnectionError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{database_url}");

    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(e),
    }
}

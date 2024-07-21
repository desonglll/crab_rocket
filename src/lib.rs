use diesel::prelude::*;
use std::env;

pub mod env_variables;

pub fn establish_pg_connection() -> Result<PgConnection, diesel::result::ConnectionError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(e),
    }
}

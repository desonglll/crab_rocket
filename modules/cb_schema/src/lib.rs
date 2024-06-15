use std::env;

use diesel::{Connection, PgConnection};

pub mod schema;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn establish_pg_connection() -> Result<PgConnection, diesel::result::ConnectionError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use std::env;

use colored::Colorize;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenv::dotenv;

pub mod common;
pub mod controllers;
pub mod mappers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod update_reload;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> DbPool {
    println!("{}", "establishing pool".yellow());
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

// pub fn establish_pg_connection() -> Result<PgConnection, diesel::result::ConnectionError> {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     println!("{} {}", "Current DATABASE_URL: \t".green(), database_url.to_string().blue());

//     match PgConnection::establish(&database_url) {
//         Ok(conn) => Ok(conn),
//         Err(e) => Err(e),
//     }
// }
pub fn establish_pg_connection(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
    println!("{}", "using pool".green());
    pool.get()
}
// pub fn establish_test_pg_connection() -> State<Pool<ConnectionManager<PgConnection>>> {
//     State::<DbPool>::from(&establish_pool())
// }

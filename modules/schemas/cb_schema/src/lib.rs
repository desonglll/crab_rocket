use std::{env, error::Error, fs};

use colored::Colorize;
use diesel::{
    connection::SimpleConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
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
    println!("{}", "Establishing Pool".blue());
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool.");
    println!("{}", "Establishing Pool Successfully.".blue());
    pool
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
    println!("{}", "Using Database Pool".green());
    pool.get()
}
// pub fn establish_test_pg_connection() -> State<Pool<ConnectionManager<PgConnection>>> {
//     State::<DbPool>::from(&establish_pool())
// }

// DO NOT USE!!!
pub fn run_migrations(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<(), Box<dyn Error>> {
    // 获取迁移文件夹的路径
    let migrations_path = "migrations";
    let mut conn = establish_pg_connection(pool).unwrap();

    // 遍历迁移文件夹中的所有迁移文件
    for entry in fs::read_dir(migrations_path)? {
        let entry = entry?;
        let path = entry.path();

        // 确保只处理 SQL 文件
        if path.extension().map_or(false, |ext| ext == "sql") {
            let sql = fs::read_to_string(&path)?;
            conn.batch_execute(&sql)?;
        }
    }

    Ok(())
}

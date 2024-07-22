use crate::models::info::Info;
use crab_rocket_schema::schema::employee_table::dsl::*;
use crab_rocket_schema::schema::post_table::dsl::*;
use crab_rocket_schema::schema::task_table::dsl::*;
use crab_rocket_schema::schema::user_table::dsl::*;
use diesel::dsl::count_star;
use diesel::RunQueryDsl;
use diesel::{PgConnection, QueryDsl};

pub fn get_info(conn: &mut PgConnection) -> Result<Info, diesel::result::Error> {
    let post_count: i64 = post_table.select(count_star()).first(conn)?;
    let employee_count: i64 = employee_table.select(count_star()).first(conn)?;
    let task_count: i64 = task_table.select(count_star()).first(conn)?;
    let user_count: i64 = user_table.select(count_star()).first(conn)?;

    let info = Info::new(post_count, employee_count, task_count, user_count);
    Ok(info)
}

#[cfg(test)]
mod test {
    use super::get_info;
    use crab_rocket_schema::{establish_pg_connection, establish_pool, DbPool};
    use rocket::State; // 建立数据库连接

    #[test]
    fn test_get_info() {
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        match establish_pg_connection(pool) {
            Ok(mut conn) => match get_info(&mut conn) {
                Ok(data) => {
                    println!("{data:?}");
                    ()
                }
                Err(e) => {
                    println!("{e:?}");
                    ()
                }
            },
            Err(e) => {
                println!("{e:?}");
                ()
            }
        }
    }
}

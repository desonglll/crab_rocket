use crate::models::info::Info;
use crate::schema::employee_table::dsl::*;
use crate::schema::posts::dsl::*;
use crate::schema::tasks::dsl::*;
use crate::schema::user_table::dsl::*;
use diesel::dsl::count_star;
use diesel::RunQueryDsl;
use diesel::{PgConnection, QueryDsl};

pub fn get_info(conn: &mut PgConnection) -> Result<Info, diesel::result::Error> {
    let post_count: i64 = posts.select(count_star()).first(conn)?;
    let employee_count: i64 = employee_table.select(count_star()).first(conn)?;
    let task_count: i64 = tasks.select(count_star()).first(conn)?;
    let user_count: i64 = user_table.select(count_star()).first(conn)?;

    let info = Info::new(post_count, employee_count, task_count, user_count);
    Ok(info)
}

#[cfg(test)]
mod test {
    use crate::establish_pg_connection;

    use super::get_info;

    #[test]
    fn test_get_info() {
        match establish_pg_connection() {
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

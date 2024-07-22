use diesel::prelude::*;
use rocket::State;

use crate::{establish_pg_connection, schema, DbPool};
use colored::Colorize;
pub fn update_reload_count(pool: &State<DbPool>) {
    use self::schema::reload_counts::dsl::*;
    use chrono::Local;
    use diesel::dsl::insert_into;

    let today = Local::now().date_naive();

    let new_reload = crate::models::reload_count::ReloadCount {
        reload_date: today,
        count: 1,
    };
    println!("{} {}", "Reload Operation: \t".green(), new_reload);
    match establish_pg_connection(pool) {
        Ok(mut conn) => {
            insert_into(reload_counts)
                .values(&new_reload)
                .on_conflict(reload_date)
                .do_update()
                .set(count.eq(count + 1))
                .execute(&mut conn)
                .expect("Error updating reload count");
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
}

#[cfg(test)]
mod test {
    use rocket::State;

    use crate::{establish_pool, DbPool};

    #[test]
    fn test_update_reload_count() {
        use super::update_reload_count;
        // Clear environment variable before running.
        std::env::remove_var("DATABASE_URL");
        dotenv::dotenv().ok();
        let binding = establish_pool();
        let pool = State::<DbPool>::from(&binding);
        update_reload_count(pool)
    }
}

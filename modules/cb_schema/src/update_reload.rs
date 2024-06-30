use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;

use crate::{establish_pg_connection, schema};

#[derive(Serialize, Insertable, Queryable, AsChangeset, Identifiable)]
#[diesel(table_name = schema::reload_counts)]
#[diesel(primary_key(reload_date))]
pub struct ReloadCount {
    reload_date: NaiveDate,
    count: i32,
}

pub fn update_reload_count() {
    use self::schema::reload_counts::dsl::*;
    use chrono::Local;
    use diesel::dsl::insert_into;

    let today = Local::now().date_naive();

    let new_reload = ReloadCount {
        reload_date: today,
        count: 1,
    };
    match establish_pg_connection() {
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

mod test {

    #[test]
    fn test_update_reload_count() {
        use super::update_reload_count;
        // Clear environment variable before running.
        std::env::remove_var("DATABASE_URL");
        dotenv::dotenv().ok();
        update_reload_count()
    }
}

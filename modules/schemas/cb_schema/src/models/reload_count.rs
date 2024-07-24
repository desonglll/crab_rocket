use chrono::NaiveDate;
use colored::Colorize;
use diesel::prelude::*;
use serde::Serialize;

use crate::schema;

#[derive(Serialize, Insertable, Queryable, AsChangeset, Identifiable)]
#[diesel(table_name = schema::reload_counts)]
#[diesel(primary_key(reload_date))]
pub struct ReloadCount {
    pub reload_date: NaiveDate,
    pub count: i32,
}

impl std::fmt::Display for ReloadCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Reload Date: \t{}, Count: \t{}",
            self.reload_date.to_string().blue(),
            self.count.to_string().blue()
        )
    }
}

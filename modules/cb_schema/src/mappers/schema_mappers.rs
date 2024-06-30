use crate::schema::reload_counts::{self, dsl::*};
use crate::update_reload::ReloadCount;
use diesel::prelude::*;
use diesel::PgConnection;
pub fn get_reload_counts(
    conn: &mut PgConnection,
) -> Result<Vec<ReloadCount>, diesel::result::Error> {
    reload_counts.order(reload_counts::reload_date.asc()).load::<ReloadCount>(conn)
}

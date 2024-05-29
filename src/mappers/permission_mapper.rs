use crate::models::permission::Permission;
// use crate::schema::permission_table::dsl::*;
use crate::schema::permission_table::{self};
use diesel::prelude::*;

pub fn fetch_all_permissions(
    conn: &mut PgConnection,
) -> Result<Vec<Permission>, diesel::result::Error> {
    permission_table::table.order(permission_table::id.asc()).load::<Permission>(conn)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fetch_all_roles() {
        use super::*;
        use crate::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => {
                match fetch_all_permissions(&mut conn) {
                    Ok(permissions) => {
                        println!("{permissions:?}");
                        ()
                    }
                    Err(_) => (),
                }
            }
            Err(_) => (),
        }
    }
}

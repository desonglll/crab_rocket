use crate::{
    models::role::{NewRole, Role},
    schema::role_table,
};
use diesel::prelude::*;

pub fn insert_role(
    conn: &mut PgConnection,
    new_role: &NewRole,
) -> Result<Role, diesel::result::Error> {
    match diesel::insert_into(role_table::table)
        .values(new_role)
        .returning(Role::as_returning())
        .get_result(conn)
    {
        Ok(inserted_role) => Ok(inserted_role),
        Err(e) => {
            println!("{e:?}");
            Err(e)
        }
    }
}

pub fn fetch_all_roles(conn: &mut PgConnection) -> Result<Vec<Role>, diesel::result::Error> {
    role_table::table
        .order(role_table::role_id.asc())
        .load::<Role>(conn)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_insert_role() {
        use super::*;
        use crate::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => {
                // 创建一个新的 NewPost 实例
                let new_role = NewRole {
                    role_name: String::from("admin"),
                    description: Some(String::from("Administrator role with full access")),
                    permissions: Some(String::from("admin:full_access,user:view,post:edit")),
                };
                let _ = insert_role(&mut conn, &new_role);
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_fetch_all_roles() {
        use super::*;
        use crate::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => match fetch_all_roles(&mut conn) {
                Ok(roles) => {
                    println!("{roles:?}");
                    ()
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }
}

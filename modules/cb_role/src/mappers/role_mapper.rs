use crate::models::role::{NewRole, Role};
use crab_rocket_schema::schema::role_table::dsl::*; //配合下面的 `posts.filter()`
use crab_rocket_schema::schema::role_table::{self};
use crab_rocket_utils::time::get_e8_time;
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
    role_table::table.order(role_table::role_id.asc()).load::<Role>(conn)
}

pub fn delete_role_by_id(conn: &mut PgConnection, id: i32) -> Result<Role, diesel::result::Error> {
    diesel::delete(role_table.filter(role_id.eq(id))).get_result(conn)
}

pub fn fetch_role_by_id(conn: &mut PgConnection, id: i32) -> Result<Role, diesel::result::Error> {
    // 配合 use crate::schema::posts::dsl::*;
    role_table.filter(role_id.eq(id)).first(conn)
}

pub fn update_role_by_id(
    conn: &mut PgConnection,
    id: i32,
    role: &crate::models::role::PatchRole,
) -> Result<Role, diesel::result::Error> {
    diesel::update(role_table.filter(role_id.eq(id)))
        .set((
            role_name.eq(role.role_name()),
            description.eq(role.description()),
            permissions.eq(role.permissions()),
            created_at.eq(role.created_at()),
            updated_at.eq(get_e8_time()),
        ))
        .get_result(conn)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_insert_role() {
        use super::*;
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => {
                // 创建一个新的 NewPost 实例
                let new_role = NewRole::new(
                    String::from("admin"),
                    Some(String::from("Administrator role with full access")),
                    Some(String::from("admin:full_access,user:view,post:edit")),
                );
                let _ = insert_role(&mut conn, &new_role);
            }
            Err(_) => (),
        }
    }

    #[test]
    fn test_fetch_all_roles() {
        use super::*;
        use crab_rocket_schema::establish_pg_connection; // 建立数据库连接
        match establish_pg_connection() {
            Ok(mut conn) => {
                match fetch_all_roles(&mut conn) {
                    Ok(roles) => {
                        println!("{roles:?}");
                        ()
                    }
                    Err(_) => (),
                }
            }
            Err(_) => (),
        }
    }
}

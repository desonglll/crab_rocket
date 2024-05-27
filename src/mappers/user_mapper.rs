use crate::models::user::{NewUser, PatchUser, User};
use crate::schema::user_table::dsl::*;
use crate::schema::user_table::{self};
use crate::utils::time::get_e8_time;
use diesel::prelude::*;

// GOOD:
pub fn insert_user(conn: &mut PgConnection, user: &NewUser) -> Result<User, diesel::result::Error> {
    let new_user = diesel::insert_into(user_table)
        .values(user)
        .returning(User::as_returning())
        .get_result(conn);
    match new_user {
        Ok(new_user) => Ok(new_user),
        Err(e) => Err(e),
    }
}

// GOOD:
pub fn fetch_all_users(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    user_table.order(user_table::user_id.asc()).load::<User>(conn)
}

// GOOD:
pub fn fetch_user_by_id(conn: &mut PgConnection, id: i32) -> Result<User, diesel::result::Error> {
    user_table.filter(user_table::user_id.eq(id)).first(conn)
}

// GOOD:
pub fn update_user_by_id(
    conn: &mut PgConnection,
    id: i32,
    user: &PatchUser,
) -> Result<User, diesel::result::Error> {
    diesel::update(user_table.filter(user_id.eq(id)))
        .set((
            user_table::username.eq(user.username()),
            user_table::password.eq(user.password()),
            user_table::role.eq(user.role()),
            user_table::email.eq(user.email()),
            user_table::fullname.eq(user.fullname()),
            user_table::avatar_url.eq(user.avatar_url()),
            user_table::bio.eq(user.bio()),
            user_table::updated_at.eq(Some(get_e8_time())),
            user_table::mobile_phone.eq(user.mobile_phone()),
            user_table::created_at.eq(user.created_at()),
        ))
        .get_result(conn)
}

// GOOD:
pub fn delete_user_by_id(conn: &mut PgConnection, id: i32) -> Result<User, diesel::result::Error> {
    diesel::delete(user_table.filter(user_table::user_id.eq(id))).get_result(conn)
}

#[cfg(test)]
mod test {
    use crate::models::user::PatchUser;

    #[test]
    fn test_insert_user() {
        use crate::{
            establish_pg_connection, mappers::user_mapper::insert_user, models::user::NewUser,
            utils::time::get_e8_time,
        };

        let user = NewUser::new(
            "username".to_string(),
            Some(1),
            Some(get_e8_time()),
            Some(String::from("email")),
            "password".to_string(),
            Some(String::from("fullname")),
            Some(String::from("avatar_url")),
            Some(String::from("bio")),
            Some(get_e8_time()),
            "mobile_phone".to_string(),
        );
        println!("{user:?}");
        match establish_pg_connection() {
            Ok(mut conn) => match insert_user(&mut conn, &user) {
                Ok(result) => println!("{result}"),
                Err(_) => println!("err"),
            },
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_fetch_all_users() {
        use crate::establish_pg_connection;

        use super::fetch_all_users;
        match establish_pg_connection() {
            Ok(mut conn) => match fetch_all_users(&mut conn) {
                Ok(res) => println!("{res:?}"),
                Err(_) => println!("Err"),
            },
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_fetch_user_by_id() {
        use super::fetch_user_by_id;
        use crate::establish_pg_connection;
        let id = 1;
        match establish_pg_connection() {
            Ok(mut conn) => match fetch_user_by_id(&mut conn, id) {
                Ok(res) => println!("{res}"),
                Err(_) => println!("Err"),
            },
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_update_user_by_id() {
        match crate::establish_pg_connection() {
            Ok(mut conn) => {
                let id = 1;
                let user = PatchUser::default();
                match crate::mappers::user_mapper::update_user_by_id(&mut conn, id, &user) {
                    Ok(res) => println!("{res}"),
                    Err(_) => println!("Err"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }

    #[test]
    fn test_delete_user_by_id() {
        match crate::establish_pg_connection() {
            Ok(mut conn) => {
                let id = 1;
                let result = super::delete_user_by_id(&mut conn, id);
                match result {
                    Ok(res) => println!("{res}"),
                    Err(_) => println!("Err"),
                }
            }
            Err(_) => println!("establish_pg_connection error"),
        }
    }
}

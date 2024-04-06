use crate::models::user::{NewUser, PatchUser, User};
use crate::schema::users::dsl::*;
use crate::schema::users::{self};
use crate::utils::time::get_e8_time;
use diesel::prelude::*;

pub fn insert_user(conn: &mut PgConnection, user: &NewUser) -> Result<User, diesel::result::Error> {
    let new_user = diesel::insert_into(users)
        .values(user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user");
    Ok(new_user)
}

pub fn fetch_all_users(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    users.order(users::user_id.asc()).load::<User>(conn)
}

pub fn fetch_user_by_id(conn: &mut PgConnection, id: i32) -> Result<User, diesel::result::Error> {
    users.filter(users::user_id.eq(id)).first(conn)
}

pub fn update_user_by_id(
    conn: &mut PgConnection,
    id: i32,
    user: &PatchUser,
) -> Result<User, diesel::result::Error> {
    diesel::update(users.filter(user_id.eq(id)))
        .set((
            users::username.eq(user.username.clone()),
            users::password.eq(user.password.clone()),
            users::role.eq(user.role.clone()),
            users::email.eq(user.email.clone()),
            users::fullname.eq(user.fullname.clone()),
            users::avatar_url.eq(user.avatar_url.clone()),
            users::bio.eq(user.bio.clone()),
            users::updated_at.eq(Some(get_e8_time())),
            users::mobile_phone.eq(user.mobile_phone.clone()),
        ))
        .get_result(conn)
}

pub fn delete_user_by_id(conn: &mut PgConnection, id: i32) -> Result<User, diesel::result::Error> {
    diesel::delete(users.filter(users::user_id.eq(id))).get_result(conn)
}
#[cfg(test)]
mod test {

    #[test]
    fn test_insert_user() {
        use crate::{
            establish_pg_connection, mappers::user_mapper::insert_user, models::user::NewUser,
            utils::time::get_e8_time,
        };

        let user = NewUser::new(
            "username".to_string(),
            Some(String::from("role")),
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
            Ok(mut conn) => {
                let result = insert_user(&mut conn, &user).unwrap();
                println!("{result}");
            }
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
        use crate::models::user::User;
        match crate::establish_pg_connection() {
            Ok(mut conn) => {
                let id = 1;
                let user = User::new_empty();
                match crate::mappers::user_mapper::update_user_by_id(&mut conn, id, &user.into()) {
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

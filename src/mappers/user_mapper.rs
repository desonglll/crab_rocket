use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::schema::users::{self};
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
        let mut conn = establish_pg_connection();
        let result = insert_user(&mut conn, &user).unwrap();
        println!("{result}");
    }

    #[test]
    fn test_fetch_all_users() {
        use crate::establish_pg_connection;

        use super::fetch_all_users;
        let mut conn = establish_pg_connection();
        let all_users = fetch_all_users(&mut conn).unwrap();
        println!("{all_users:?}");
    }
}

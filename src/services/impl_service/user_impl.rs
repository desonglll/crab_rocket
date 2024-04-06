use crate::{
    establish_pg_connection, mappers::user_mapper, models::user::User, services::user_service,
};

impl user_service::GetUser for User {
    fn insert_single_user(
        user: &crate::models::user::NewUser,
    ) -> Result<User, diesel::result::Error> {
        let mut conn = establish_pg_connection();
        match user_mapper::insert_user(&mut conn, user) {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }
    fn get_all_users() -> Result<Vec<User>, diesel::result::Error> {
        let mut conn = establish_pg_connection();
        match user_mapper::fetch_all_users(&mut conn) {
            Ok(all_tasks) => Ok(all_tasks),
            Err(e) => Err(e),
        }
    }
}

mod test {

    #[test]
    fn test_insert_single_user() {
        use crate::services::user_service::GetUser;
        use crate::{
            models::user::{NewUser, User},
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
        let result = User::insert_single_user(&user).unwrap();
        println!("{result}");
    }

    #[test]
    fn test_get_all_users() {
        use crate::models::user::User;
        use crate::services::user_service::GetUser;
        let result = User::get_all_users().unwrap();
        println!("{result:?}");
    }
}

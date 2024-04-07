use crate::{
    establish_pg_connection,
    mappers::user_mapper,
    models::user::{PatchUser, User},
    services::user_service,
};

impl user_service::GetUser for User {
    fn insert_single_user(
        user: &crate::models::user::NewUser,
    ) -> Result<User, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match user_mapper::insert_user(&mut conn, user) {
                Ok(user) => Ok(user),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
    fn get_all_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match user_mapper::fetch_all_users(&mut conn) {
                Ok(all_users) => Ok(all_users),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }

    fn get_user_by_id(id: i32) -> Result<User, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match user_mapper::fetch_user_by_id(&mut conn, id) {
                Ok(user) => Ok(user),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
    fn update_user_by_id(id: i32, user: &PatchUser) -> Result<User, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match user_mapper::update_user_by_id(&mut conn, id, user) {
                Ok(updated_user) => Ok(updated_user),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
    fn delete_user_by_id(id: i32) -> Result<User, Box<dyn std::error::Error>> {
        match establish_pg_connection() {
            Ok(mut conn) => match user_mapper::delete_user_by_id(&mut conn, id) {
                Ok(deleted_user) => Ok(deleted_user),
                Err(e) => {
                    println!("{e:?}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                println!("{e:?}");
                Err(Box::new(e))
            }
        }
    }
}
#[cfg(test)]
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
        match User::insert_single_user(&user) {
            Ok(result) => println!("{result}"),
            Err(_) => println!("Err"),
        }
    }

    #[test]
    fn test_get_all_users() {
        use crate::models::user::User;
        use crate::services::user_service::GetUser;
        match User::get_all_users() {
            Ok(res) => println!("{res:?}"),
            Err(_) => println!("Err"),
        }
    }

    #[test]
    fn test_get_user_by_id() {
        use crate::models::user::User;
        use crate::services::user_service::GetUser;
        match User::get_user_by_id(1) {
            Ok(res) => println!("{res:?}"),
            Err(_) => println!("Err"),
        }
    }

    #[test]
    fn test_delete_user_by_id() {
        use crate::models::user::User;
        use crate::services::user_service::GetUser;
        match User::delete_user_by_id(2) {
            Ok(res) => println!("{res:?}"),
            Err(_) => println!("Err"),
        }
    }
}

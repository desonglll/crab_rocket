use crate::{
    models::user::{NewUser, User},
    services::user_service::GetUser,
};

pub fn insert_single_user_controller(user: &NewUser) -> (i32, &'static str, User) {
    match User::insert_single_user(user) {
        Ok(result) => (200, "INSERT OK", result),
        Err(_) => (204, "INSERT ERROR", User::new_empty()),
    }
}

pub fn get_all_users_controller() -> (i32, &'static str, Vec<User>) {
    match User::get_all_users() {
        Ok(all_users) => (200, "GET ALL USERS OK", all_users),
        Err(_) => (204, "GET ALL USERS ERROR", Vec::new()),
    }
}

mod test {

    #[test]
    fn test_insert_single_user_controller() {
        use crate::{models::user::NewUser, utils::time::get_e8_time};

        use super::insert_single_user_controller;
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
        let (code, message, user) = insert_single_user_controller(&user);
        println!("{code}\n{message}\n{user}\n");
    }

    #[test]
    fn test_get_all_users_controller() {
        use super::get_all_users_controller;
        let (code, message, all_users) = get_all_users_controller();
        println!("{code}\n{message}\n{all_users:?}\n");
    }
}

use crate::{
    models::user::{NewUser, PatchUser, User},
    services::user_service::GetUser,
};

pub fn insert_single_user_controller(user: &NewUser) -> (i32, String, User) {
    match User::insert_single_user(user) {
        Ok(result) => (200, "INSERT OK".to_string(), result),
        Err(e) => (204, e.to_string(), User::default()),
    }
}

pub fn get_all_users_controller() -> (i32, String, Vec<User>) {
    match User::get_all_users() {
        Ok(all_users) => (200, String::from("GET ALL USERS OK"), all_users),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}

pub fn get_user_by_id_controller(id: i32) -> (i32, String, User) {
    match User::get_user_by_id(id) {
        Ok(user) => (200, String::from("GET USER BY ID OK"), user),
        Err(e) => (204, e.to_string(), User::default()),
    }
}

pub fn update_user_by_id_controller(id: i32, user: &PatchUser) -> (i32, String, User) {
    match User::update_user_by_id(id, user) {
        Ok(updated_user) => (200, String::from("UPDATED USER OK"), updated_user),
        Err(e) => (204, e.to_string(), User::default()),
    }
}

pub fn delete_user_by_id_controller(id: i32) -> (i32, String, User) {
    match User::delete_user_by_id(id) {
        Ok(deleted_user) => (200, String::from("DELETE USER OK"), deleted_user),
        Err(e) => (204, e.to_string(), User::default()),
    }
}
#[cfg(test)]
mod test {
    use crate::models::user::PatchUser;

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

    #[test]
    fn test_get_user_by_id_controller() {
        use super::get_user_by_id_controller;
        let (code, message, user) = get_user_by_id_controller(1);
        println!("{code}\n{message}\n{user}\n");
    }

    #[test]
    fn test_update_user_by_id_controller() {
        use super::update_user_by_id_controller;
        use crate::utils::time::get_e8_time;
        let user = PatchUser::new(
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
        let (code, message, updated_user) = update_user_by_id_controller(1, &user);
        println!("{code}\n{message}\n{updated_user}\n");
    }

    #[test]
    fn test_delete_user_by_id_controller() {
        let (code, message, updated_user) = super::delete_user_by_id_controller(3);
        println!("{code}\n{message}\n{updated_user}\n");
    }
}

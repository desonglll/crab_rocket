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

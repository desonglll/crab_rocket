use crate::models::role::PatchRole;
use crate::{
    models::role::{NewRole, Role},
    services::role_service::GetRole,
};

pub fn insert_role_controller(new_role: &mut NewRole) -> (i32, String, Role) {
    match Role::insert_role(&new_role.clone()) {
        Ok(result_task) => (201, String::from("Created"), result_task),
        Err(e) => (204, e.to_string(), Role::default()),
    }
}

pub fn get_all_roles_controller() -> (i32, String, Vec<Role>) {
    match Role::get_all_roles() {
        Ok(all_roles) => (200, String::from("found successfully"), all_roles),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}

pub fn delete_role_by_id_controller(id: i32) -> (i32, String, Role) {
    match Role::delete_role_by_id(id) {
        Ok(deleted_role) => (200, String::from("DELETE ROLE BY ID OK"), deleted_role),
        Err(e) => {
            println!("{e:?}");
            (204, e.to_string(), Role::default())
        }
    }
}

pub fn get_role_by_id_controller(id: i32) -> (i32, String, Role) {
    match Role::get_role_by_id(id) {
        Ok(role) => (200, String::from("GET ROLE BY ID OK"), role),
        Err(e) => {
            println!("{e:?}");
            (204, e.to_string(), Role::default())
        }
    }
}

pub fn update_role_by_id_controller(id: i32, role: &PatchRole) -> (i32, String, Role) {
    match Role::update_role_by_id(id, role) {
        Ok(updated_role) => (200, String::from("UPDATE ROLE BY ID OK"), updated_role),
        Err(e) => {
            println!("{e:?}");
            (204, e.to_string(), Role::default())
        }
    }
}

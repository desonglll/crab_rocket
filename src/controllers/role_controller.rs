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

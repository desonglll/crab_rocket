use crate::{models::permission::Permission, services::permission_service::GetPermission};

pub fn get_all_permissions_controller() -> (i32, String, Vec<Permission>) {
    match Permission::get_all_permissions() {
        Ok(all_permissions) => (200, String::from("found successfully"), all_permissions),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}

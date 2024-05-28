use crate::models::permission::Permission;

pub trait GetPermission {
    fn get_all_permissions() -> Result<Vec<Permission>, Box<dyn std::error::Error>>;
}

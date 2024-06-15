use crate::models::role::{NewRole, PatchRole, Role};

pub trait GetRole {
    fn insert_role(role: &NewRole) -> Result<Role, Box<dyn std::error::Error>>;
    fn get_all_roles() -> Result<Vec<Role>, Box<dyn std::error::Error>>;
    fn delete_role_by_id(id: i32) -> Result<Role, Box<dyn std::error::Error>>;
    fn get_role_by_id(id: i32) -> Result<Role, Box<dyn std::error::Error>>;
    fn update_role_by_id(id: i32, role: &PatchRole) -> Result<Role, Box<dyn std::error::Error>>;
}

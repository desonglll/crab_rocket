use crate::models::role::{NewRole, Role};

pub trait GetRole {
    fn insert_role(role: &NewRole) -> Result<Role, Box<dyn std::error::Error>>;
    fn get_all_roles() -> Result<Vec<Role>, Box<dyn std::error::Error>>;
}

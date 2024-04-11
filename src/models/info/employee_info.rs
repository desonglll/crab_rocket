use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EmployeeInfo {
    pub count: i64,
}

impl EmployeeInfo {
    pub fn new_empty() -> Self { EmployeeInfo { count: -1 } }
}

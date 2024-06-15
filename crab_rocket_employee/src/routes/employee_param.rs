use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct EmployeeParam {
    pub employee_id: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

use rocket::{get, post, serde::json::Json};
use serde_json::json;

use crate::{controllers::role_controller, models::role::NewRole};

#[post("/role", data = "<role>")]
pub fn insert_role(role: Json<NewRole>) -> Json<serde_json::Value> {
    let (status, message, role) = role_controller::insert_role_controller(&mut role.clone());
    let response = json!(
        {
            "status": status,
            "message": message,
            "data": role
        }
    );
    Json(serde_json::from_value(response).unwrap())
}

#[get("/role")]
pub fn get_all_roles() -> Json<serde_json::Value> {
    let (status, message, roles) = role_controller::get_all_roles_controller();
    let response = json!(
        {
            "status": status,
            "message": message,
            "data": roles
        }
    );
    Json(serde_json::from_value(response).unwrap())
}

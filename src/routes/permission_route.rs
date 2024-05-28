use crate::controllers::permission_controller;
use rocket::{get, serde::json::Json};
use serde_json::json;

#[get("/permission")]
pub fn get_all_permissions() -> Json<serde_json::Value> {
    let (status, message, permissions) = permission_controller::get_all_permissions_controller();
    let response = json!(
        {
            "status": status,
            "message": message,
            "data":permissions
        }
    );
    Json(serde_json::from_value(response).unwrap())
}

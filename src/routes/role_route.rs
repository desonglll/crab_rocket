use rocket::{delete, get, patch, post, serde::json::Json};
use serde_json::json;

use crate::models::role::PatchRole;
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

#[delete("/role/<id>")]
pub fn delete_role_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, deleted_role) = role_controller::delete_role_by_id_controller(id);
    let response = json!({
        "code": code,
        "message": message,
        "data": deleted_role
    });
    Json(response)
}

#[get("/role/<id>")]
pub fn get_role_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, role) = role_controller::get_role_by_id_controller(id);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":role
    }))
    .unwrap();
    Json(response)
}

#[patch("/role/<id>", data = "<role>")]
pub fn update_role_by_id(id: i32, role: Json<PatchRole>) -> Json<serde_json::Value> {
    let (code, message, updated_role) = role_controller::update_role_by_id_controller(id, &role);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":updated_role
    }))
    .unwrap();
    Json(response)
}

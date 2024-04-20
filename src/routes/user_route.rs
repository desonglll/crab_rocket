use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use serde_json::json;

use crate::{
    controllers::user_controller,
    models::user::{NewUser, PatchUser},
};

#[utoipa::path(
    responses(
        (status = 200, description = "found successfully", body = Vec<User>),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[get("/user")]
pub fn get_all_users() -> Json<serde_json::Value> {
    let (code, message, all_users) = user_controller::get_all_users_controller();
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":all_users
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "found successfully", body = User),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[get("/user/<id>")]
pub fn get_user_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, user) = user_controller::get_user_by_id_controller(id);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":user
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "created successfully", body = User),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[post("/user", data = "<user>")]
pub fn insert_single_user(user: Json<NewUser>) -> Json<serde_json::Value> {
    let (code, message, inserted_user) = user_controller::insert_single_user_controller(&user);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":inserted_user
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "updated successfully", body = User),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[patch("/user/<id>", data = "<user>")]
pub fn update_user_by_id(id: i32, user: Json<PatchUser>) -> Json<serde_json::Value> {
    let (code, message, updated_user) = user_controller::update_user_by_id_controller(id, &user);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":updated_user
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "delete successfully", body = User),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[delete("/user/<id>")]
pub fn delete_user_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, updated_user) = user_controller::delete_user_by_id_controller(id);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":updated_user
    }))
    .unwrap();
    Json(response)
}
#[options("/user")]
pub fn options_user_filter() -> Status {
    Status::Ok
}

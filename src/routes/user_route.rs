use rocket::{delete, get, patch, post, serde::json::Json};
use serde_json::json;

use crate::{
    controllers::user_controller,
    models::user::{NewUser, PatchUser},
};

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

#[delete("/user/<id>")]
pub fn delete_user_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, updated_user) = user_controller::delete_user_by_id_controller(id);
    let response = serde_json::from_value(json!({         "code":code,
        "message":message,
        "data":updated_user
    }))
    .unwrap();
    Json(response)
}

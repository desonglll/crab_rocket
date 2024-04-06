use rocket::{get, post, serde::json::Json};
use serde_json::json;

use crate::{controllers::user_controller, models::user::NewUser};

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

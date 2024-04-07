use rocket::{delete, get, post, serde::json::Json};
use serde_json::json;

use crate::{controllers::follow_controller, models::follow::NewFollow};
#[get("/follow")]
pub fn get_all_follows() -> Json<serde_json::Value> {
    let (code, message, all_follows) = follow_controller::get_all_follows_controller();
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":all_follows
    }))
    .unwrap();
    Json(response)
}

#[get("/follow?<following_uid>&<followed_uid>")]
pub fn get_follows_by_id(following_uid: i32, followed_uid: i32) -> Json<serde_json::Value> {
    let (code, message, follows) =
        follow_controller::get_follows_by_user_id_controller(following_uid, followed_uid);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":follows
    }))
    .unwrap();
    Json(response)
}

#[post("/follow", data = "<follow>")]
pub fn insert_single_follow(follow: Json<NewFollow>) -> Json<serde_json::Value> {
    let (code, message, inserted_follow) = follow_controller::create_new_follow_controller(&follow);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":inserted_follow
    }))
    .unwrap();
    Json(response)
}

#[delete("/follow", data = "<follow>")]
pub fn delete_follow(follow: Json<NewFollow>) -> Json<serde_json::Value> {
    let (code, message, deleted_follow) = follow_controller::delete_follow_controller(&follow);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":deleted_follow
    }))
    .unwrap();
    Json(response)
}

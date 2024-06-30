use rocket::{delete, get, post, serde::json::Json};
use serde_json::json;

use crate::{controllers::follow_controller, models::follow::NewFollow};

#[utoipa::path(
    responses(
        (status = 200, description = "found successfully", body = Follow),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[get("/follow")]
pub fn get_all_follows() -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let (code, message, all_follows) = follow_controller::get_all_follows_controller();
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":all_follows
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "found successfully", body = Vec<Follow>),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/follow/filter", data = "<params>")]
pub fn get_follows_by_params(
    params: Json<crate::routes::follow_param::FollowParam>,
) -> Json<serde_json::Value> {
    let (code, message, follows) = follow_controller::get_follows_by_params_controller(&params);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":follows
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "created successfully", body = Follow),
        (status = NOT_FOUND, description = "not found")
    )
)]
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

#[utoipa::path(
    responses(
        (status = 200, description = "delete successfully", body = Follow),
        (status = NOT_FOUND, description = "not found")
    )
)]
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

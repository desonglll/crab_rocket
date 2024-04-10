use rocket::{delete, form::FromForm, get, http::Status, options, patch, post, serde::json::Json};
use serde_json::json;

use crate::{
    controllers::post_controller,
    models::post::{NewPost, PatchPost},
};

use super::models::post_param::PostParam;

#[derive(FromForm)]
pub struct PostQuery {
    pub author_id: Option<i32>,
    pub category: Option<String>,
    // 其他可能的查询条件
}

#[get("/post")]
pub fn get_all_posts() -> Json<serde_json::Value> {
    let (code, message, all_posts) = post_controller::get_all_posts_controller();
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":all_posts
    }))
    .unwrap();
    Json(response)
}

#[get("/post/<id>")]
pub fn get_post_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, post) = post_controller::get_post_by_id_controller(id);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":post
    }))
    .unwrap();
    Json(response)
}

#[patch("/post/<id>", data = "<post>")]
pub fn update_post_by_id(id: i32, post: Json<PatchPost>) -> Json<serde_json::Value> {
    let (code, message, updated_post) = post_controller::update_post_by_id_controller(id, &post);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":updated_post
    }))
    .unwrap();
    Json(response)
}

#[delete("/post/<id>")]
pub fn delete_post_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, deleted_post) = post_controller::delete_post_by_id_controller(id);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":deleted_post
    }))
    .unwrap();
    Json(response)
}

#[post("/post", data = "<post>")]
pub fn insert_single_post(post: Json<NewPost>) -> Json<serde_json::Value> {
    let (code, message, inserted_post) = post_controller::insert_single_post_controller(&post);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":inserted_post
    }))
    .unwrap();
    Json(response)
}

#[post("/post/filter", data = "<params>")]
pub fn get_posts_by_params(params: Json<PostParam>) -> Json<serde_json::Value> {
    let (code, message, (post, info)) = post_controller::get_post_by_params_controller(&params);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":{
            "posts":post,
            "info":info
        }
    }))
    .unwrap();
    Json(response)
}
#[options("/post/filter")]
pub fn options_post_filter() -> Status { Status::Ok }

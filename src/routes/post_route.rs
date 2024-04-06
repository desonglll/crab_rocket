use rocket::{post, serde::json::Json};
use serde_json::json;

use crate::{controllers::post_controller, models::post::NewPost};

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

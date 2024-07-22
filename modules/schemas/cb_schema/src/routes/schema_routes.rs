use rocket::{
    get,
    serde::json::{json, Json},
    State,
};

use crate::DbPool;

#[get("/reload_count")]
pub fn get_reload_count(pool: &State<DbPool>) -> Json<serde_json::Value> {
    // crate::update_reload::update_reload_count();
    let (code, message, data) =
        crate::controllers::reload_controller::get_reload_counts_controller(pool);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":data
    }))
    .unwrap();
    Json(response)
}

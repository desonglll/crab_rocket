use rocket::{
    get,
    serde::json::{json, Json},
};

#[get("/reload_count")]
pub fn get_reload_count() -> Json<serde_json::Value> {
    // crate::update_reload::update_reload_count();
    let (code, message, data) =
        crate::controllers::reload_controller::get_reload_counts_controller();
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":data
    }))
    .unwrap();
    Json(response)
}

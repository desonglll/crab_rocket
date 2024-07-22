use crab_rocket_schema::DbPool;
use rocket::{get, serde::json::Json, State};
use serde_json::json;

use crate::controllers::info_controller;

#[get("/info")]
pub fn get_info(pool: &State<DbPool>) -> Json<serde_json::Value> {
    let (status, message, info) = info_controller::get_info(pool);
    let response = json!(
        {
            "status": status,
            "message": message,
            "body":{
                "data":info
            }
        }
    );
    Json(serde_json::from_value(response).unwrap())
}

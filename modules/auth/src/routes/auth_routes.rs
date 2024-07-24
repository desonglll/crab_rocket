use crab_rocket_schema::DbPool;
use rocket::{post, serde::json::Json, State};

use crate::models::{login::Login, logout::Logout};

#[post("/login", data = "<param>")]
pub fn login(param: Option<Json<Login>>, pool: &State<DbPool>) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(Login::default()));
    let param = param.into_inner();
    let resp = param.login(pool).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/logout", data = "<param>")]
pub fn logout(param: Option<Json<Logout>>, pool: &State<DbPool>) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(Logout::default()));
    let param = param.into_inner();
    let resp = param.logout(pool).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::request_param::RequestParam;

use crate::controllers::user_controller::UserController;
use crate::models::user::User;
use crate::models::user_filter::UserFilter;

#[get("/user", data = "<param>")]
pub fn get_users(
    param: Option<Json<RequestParam<User, UserFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = UserController::get_all(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/user/filter", data = "<param>")]
pub fn filter_users(
    param: Option<Json<RequestParam<User, UserFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = UserController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/user/<id>", data = "<param>")]
pub fn get_user_by_id(
    param: Option<Json<RequestParam<User, UserFilter>>>,
    pool: &State<DbPool>,
    id: i32,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = UserController::get_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/user", data = "<param>")]
pub fn insert_single_user(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<User, UserFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = UserController::add_single(pool, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/user/<id>", data = "<param>")]
pub fn delete_user_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<User, UserFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let resp = UserController::delete_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/user/<id>", data = "<param>")]
pub fn update_user_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<User, UserFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = UserController::update_by_id(pool, id, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/user")]
pub fn options_user() -> Status {
    Status::Ok
}

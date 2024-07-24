use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::request_param::RequestParam;

use crate::controllers::follow_controller::FollowController;
use crate::controllers::follow_controller_trait::FollowControllerTrait;
use crate::models::follow::Follow;
use crate::models::follow_filter::FollowFilter;

#[get("/follow", data = "<param>")]
pub fn get_follows(
    param: Option<Json<RequestParam<Follow, FollowFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = FollowController::get_all(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/follow/filter", data = "<param>")]
pub fn filter_follows(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<Follow, FollowFilter>>>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = FollowController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/follow/<id>", data = "<param>")]
pub fn get_follow_by_id(
    param: Option<Json<RequestParam<Follow, FollowFilter>>>,
    pool: &State<DbPool>,
    id: i32,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = FollowController::get_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/follow", data = "<param>")]
pub fn insert_single_follow(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<Follow, FollowFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = FollowController::add_single(pool, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/follow/<id>", data = "<param>")]
pub fn delete_follow_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Follow, FollowFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let resp = FollowController::delete_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/follow/<id>", data = "<param>")]
pub fn update_follow_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Follow, FollowFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = FollowController::update_by_id(pool, id, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/follow/spec", data = "<param>")]
pub fn delete_follow_specifically(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<Follow, FollowFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = FollowController::delete_follow_specifically(pool, &mut data.into()).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}


#[options("/follow")]
pub fn options_follow() -> Status {
    Status::Ok
}

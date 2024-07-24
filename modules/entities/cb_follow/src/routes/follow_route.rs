use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;

use crate::controllers::follow_controller::FollowController;
use crate::controllers::follow_controller_trait::FollowControllerTrait;
use crate::models::follow::{PatchFollow, PostFollow};
use crate::models::follow_filter::FollowFilter;

#[get("/follow?<limit>&<offset>")]
pub fn get_follows(
    pool: &State<DbPool>,
    mut limit: Option<i32>,
    mut offset: Option<i32>,
) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(Some(PaginationParam::new(limit, offset)), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = FollowController::get_all(pool, &params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/follow/filter", data = "<param>")]
pub fn filter_follows(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<FollowFilter>>>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = FollowController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/follow/<id>")]
pub fn get_follow_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = FollowController::get_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/follow?<follower_id>&<follow_id>")]
pub fn insert_single_follow(
    pool: &State<DbPool>,
    follower_id: i32,
    follow_id: i32,
) -> Json<serde_json::Value> {
    let mut obj: PostFollow = PostFollow::new(follower_id, follow_id);
    let resp = FollowController::add_single(pool, &mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/follow/<id>")]
pub fn delete_follow_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    let resp = FollowController::delete_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/follow/<id>", data = "<task>")]
pub fn update_follow_by_id(
    pool: &State<DbPool>,
    id: i32,
    task: Json<PatchFollow>,
) -> Json<serde_json::Value> {
    let resp = FollowController::update_by_id(pool, id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/follow/spec", data = "<follow>")]
pub fn delete_follow_specifically(
    pool: &State<DbPool>,
    follow: Json<PostFollow>,
) -> Json<serde_json::Value> {
    let resp = FollowController::delete_follow_specifically(pool, &follow).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/follow", data = "<follow>")]
pub fn insert_single_follow_by_params(
    pool: &State<DbPool>,
    follow: Json<PostFollow>,
) -> Json<serde_json::Value> {
    let mut obj: PostFollow = follow.into_inner();
    let resp = FollowController::add_single(pool, &mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/follow")]
pub fn options_follow() -> Status {
    Status::Ok
}

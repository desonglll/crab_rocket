use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;

use crate::controllers::permission_controller::PermissionController;
use crate::models::permission::Permission;
use crate::models::permission_filter::PermissionFilter;

#[get("/permission?<limit>&<offset>")]
pub fn get_permissions(
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
    let resp = PermissionController::get_all(pool, &params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/permission/filter", data = "<param>")]
pub fn filter_permissions(
    param: Option<Json<RequestParam<Permission, PermissionFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = PermissionController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/permission/<id>", data = "<param>")]
pub fn get_permission_by_id(
    param: Option<Json<RequestParam<Permission, PermissionFilter>>>,
    pool: &State<DbPool>,
    id: i32,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = PermissionController::get_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/permission", data = "<param>")]
pub fn insert_single_permission(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<Permission, PermissionFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = PermissionController::add_single(pool, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/permission/<id>", data = "<param>")]
pub fn delete_permission_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Permission, PermissionFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let resp = PermissionController::delete_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/permission/<id>", data = "<param>")]
pub fn update_permission_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Permission, PermissionFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = PermissionController::update_by_id(pool, id, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/permission")]
pub fn options_permission() -> Status {
    Status::Ok
}

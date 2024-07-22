use crate::controllers::permission_controller::PermissionController;
use crate::models::permission::{PatchPermission, PostPermission};
use crate::models::permission_filter::PermissionFilter;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, options, patch, post};

/// # Note
/// 若业务逻辑复杂则启用controller层
/// 目前只是把业务逻辑简单包含在路由中
/// ## Put和Patch
/// `https://ihower.tw/blog/archives/6483`
/// PUT 比較正確的定義是 Replace (Create or Update)，
/// 例如PUT/items/1的意思是替換/items/1，如果已經存在就替換，沒有就新增。
/// PUT必須包含items/1的所有屬性資料
#[get("/permission?<limit>&<offset>")]
pub fn get_permissions(limit: Option<i32>, offset: Option<i32>) -> Json<serde_json::Value> {
    let params = RequestParam::new(Some(PaginationParam::new(limit, offset)), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = PermissionController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/permission/filter", data = "<param>")]
pub fn filter_permissions(
    param: Option<Json<RequestParam<PermissionFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(None, None)));
    let param = param.into_inner();
    println!("{param:?}");
    let resp = PermissionController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/permission/<id>")]
pub fn get_permission_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = PermissionController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/permission", data = "<permission>")]
pub fn insert_single_permission(permission: Json<PostPermission>) -> Json<serde_json::Value> {
    let mut obj: PostPermission = permission.into_inner();

    let resp = PermissionController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/permission/<id>")]
pub fn delete_permission_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = PermissionController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/permission/<id>", data = "<permission>")]
pub fn update_permission_by_id(
    id: i32,
    permission: Json<PatchPermission>,
) -> Json<serde_json::Value> {
    let resp = PermissionController::update_by_id(id, &permission).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/")]
pub fn index() -> &'static str {
    "hello world!"
}

#[options("/permission")]
pub fn options_permission() -> Status {
    Status::Ok
}

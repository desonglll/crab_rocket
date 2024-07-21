use crate::controllers::role_controller::RoleController;
use crate::models::role::{PostRole, PatchRole};
use crate::models::role_filter::RoleFilter;
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
#[get("/role?<limit>&<offset>")]
pub fn get_roles(mut limit: Option<i32>, mut offset: Option<i32>) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(PaginationParam::new(limit, offset), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = RoleController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/role/filter", data = "<param>")]
pub fn filter_roles(
    param: Option<Json<RequestParam<PaginationParam, RoleFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(PaginationParam::default(), None)));
    let param = param.into_inner();
    println!("{param:?}");
    let resp = RoleController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/role/<id>")]
pub fn get_role_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = RoleController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/role", data = "<role>")]
pub fn insert_single_role(role: Json<PostRole>) -> Json<serde_json::Value> {
    let mut obj: PostRole = role.into_inner();

    let resp = RoleController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/role/<id>")]
pub fn delete_role_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = RoleController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/role/<id>", data = "<role>")]
pub fn update_role_by_id(id: i32, role: Json<PatchRole>) -> Json<serde_json::Value> {
    let resp = RoleController::update_by_id(id, &role).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/")]
pub fn index() -> &'static str {
    "hello world!"
}

#[options("/role")]
pub fn options_role() -> Status {
    Status::Ok
}

use crate::controllers::task_controller::TaskController;
use crate::models::task::{PostTask, PatchTask};
use crate::models::task_filter::TaskFilter;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, options, patch, post};
use serde_json::json;

/// # Note
/// 若业务逻辑复杂则启用controller层
/// 目前只是把业务逻辑简单包含在路由中
/// ## Put和Patch
/// `https://ihower.tw/blog/archives/6483`
/// PUT 比較正確的定義是 Replace (Create or Update)，
/// 例如PUT/items/1的意思是替換/items/1，如果已經存在就替換，沒有就新增。
/// PUT必須包含items/1的所有屬性資料
#[get("/task?<limit>&<offset>")]
pub fn get_tasks(mut limit: Option<i32>, mut offset: Option<i32>) -> Json<serde_json::Value> {
    if limit.is_none() {
        limit = Some(10);
    };
    if offset.is_none() {
        offset = Some(0);
    };
    let params = RequestParam::new(PaginationParam::new(limit, offset), None);
    println!("{:?}", params);
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = TaskController::get_all(&params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/task/filter", data = "<param>")]
pub fn filter_tasks(
    param: Option<Json<RequestParam<PaginationParam, TaskFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::new(PaginationParam::default(), None)));
    let param = param.into_inner();
    println!("{param:?}");
    let resp = TaskController::filter(&param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/task/<id>")]
pub fn get_task_by_id(id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count();
    let resp = TaskController::get_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/task", data = "<task>")]
pub fn insert_single_task(task: Json<PostTask>) -> Json<serde_json::Value> {
    let mut obj: PostTask = task.into_inner();

    let resp = TaskController::add_single(&mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/task/<id>")]
pub fn delete_task_by_id(id: i32) -> Json<serde_json::Value> {
    let resp = TaskController::delete_by_id(id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/task/<id>", data = "<task>")]
pub fn update_task_by_id(id: i32, task: Json<PatchTask>) -> Json<serde_json::Value> {
    let resp = TaskController::update_by_id(id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/")]
pub fn index() -> &'static str {
    "hello world!"
}

#[get("/test")]
pub fn demo() -> Json<serde_json::Value> {
    let j = json!({
        "code": 200,
        "message": "success",
        "data": {
            "user": {
              "id": 123,
              "name": "John Doe",
              "email": "john@example.com"
            }
          }
    });
    Json(serde_json::from_value(j).unwrap())
}

#[options("/task/filter")]
pub fn options_task_filter() -> Status {
    Status::Ok
}

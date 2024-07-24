use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};
use rocket::State;

use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::request_param::RequestParam;

use crate::controllers::task_controller::TaskController;
use crate::models::task::Task;
use crate::models::task_filter::TaskFilter;

#[get("/task", data = "<param>")]
pub fn get_tasks(
    param: Option<Json<RequestParam<Task, TaskFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = TaskController::get_all(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/task/filter", data = "<param>")]
pub fn filter_tasks(
    param: Option<Json<RequestParam<Task, TaskFilter>>>,
    pool: &State<DbPool>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = TaskController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/task/<id>", data = "<param>")]
pub fn get_task_by_id(
    param: Option<Json<RequestParam<Task, TaskFilter>>>,
    pool: &State<DbPool>,
    id: i32,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default()));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = TaskController::get_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/task", data = "<param>")]
pub fn insert_single_task(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<Task, TaskFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = TaskController::add_single(pool, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/task/<id>", data = "<param>")]
pub fn delete_task_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Task, TaskFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let resp = TaskController::delete_by_id(pool, id, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/task/<id>", data = "<param>")]
pub fn update_task_by_id(
    pool: &State<DbPool>,
    id: i32,
    param: Option<Json<RequestParam<Task, TaskFilter>>>,
) -> Json<serde_json::Value> {
    let param = param.unwrap_or(Json(RequestParam::default())).into_inner();
    let data = param.data.clone().unwrap();
    let resp = TaskController::update_by_id(pool, id, &mut data.into(), &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/task")]
pub fn options_task() -> Status {
    Status::Ok
}

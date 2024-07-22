use crab_rocket_schema::DbPool;
use obj_traits::controller::controller_crud::ControllerCRUD;
use obj_traits::request::pagination_request_param::{PaginationParam, PaginationParamTrait};
use obj_traits::request::request_param::RequestParam;
use rocket::State;
use rocket::{delete, get, http::Status, options, patch, post, serde::json::Json};

use crate::controllers::task_controller::TaskController;
use crate::models::task::{PatchTask, PostTask};
use crate::models::task_filter::TaskFilter;

#[get("/task?<limit>&<offset>")]
pub fn get_tasks(
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
    let resp = TaskController::get_all(pool, &params).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/task/filter", data = "<param>")]
pub fn filter_tasks(
    pool: &State<DbPool>,
    param: Option<Json<RequestParam<TaskFilter>>>,
) -> Json<serde_json::Value> {
    println!("{:?}", param);
    let param = param.unwrap_or(Json(RequestParam::new(None, None)));
    let param = param.into_inner();
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = TaskController::filter(pool, &param).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[get("/task/<id>")]
pub fn get_task_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    crab_rocket_schema::update_reload::update_reload_count(pool);
    let resp = TaskController::get_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[post("/task", data = "<task>")]
pub fn insert_single_task(pool: &State<DbPool>, task: Json<PostTask>) -> Json<serde_json::Value> {
    let mut obj: PostTask = task.into_inner();

    let resp = TaskController::add_single(pool, &mut obj).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[delete("/task/<id>")]
pub fn delete_task_by_id(pool: &State<DbPool>, id: i32) -> Json<serde_json::Value> {
    let resp = TaskController::delete_by_id(pool, id).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[patch("/task/<id>", data = "<task>")]
pub fn update_task_by_id(
    pool: &State<DbPool>,
    id: i32,
    task: Json<PatchTask>,
) -> Json<serde_json::Value> {
    let resp = TaskController::update_by_id(pool, id, &task).unwrap();
    let json_value = serde_json::to_value(&resp).unwrap();
    Json(serde_json::from_value(json_value).unwrap())
}

#[options("/task")]
pub fn options_task() -> Status {
    Status::Ok
}

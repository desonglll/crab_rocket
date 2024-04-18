use crate::controllers::task_controller;
use crate::models::task::{NewTask, PatchTask, PutTask, Task};
use crate::utils::time::get_e8_time;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, options, patch, post, put};
use serde_json::json;

/// # Note
/// 若业务逻辑复杂则启用controller层
/// 目前只是把业务逻辑简单包含在路由中
/// ## Put和Patch
/// https://ihower.tw/blog/archives/6483
/// PUT 比較正確的定義是 Replace (Create or Update)，
/// 例如PUT/items/1的意思是替換/items/1，如果已經存在就替換，沒有就新增。
/// PUT必須包含items/1的所有屬性資料。

#[utoipa::path(
    responses(
        (status = 200, description = "found successfully", body = Task),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[get("/task/<id>")]
pub fn get_task_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, task) = task_controller::get_task_by_id_controller(id);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":task
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "update successfully", body = Task),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[patch("/task/<id>", data = "<task>")]
pub fn update_task_by_id(id: i32, task: Json<PatchTask>) -> Json<serde_json::Value> {
    let (code, message, patched_task) =
        task_controller::update_task_by_id_controller(id, &task.into_inner());
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":patched_task
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "put successfully", body = Task),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[put("/task/<id>", data = "<task>")]
pub fn put_task(id: i32, task: Json<PatchTask>) -> Json<serde_json::Value> {
    //Convert a patch task json to a put task json which include `id`.
    let put_task = PutTask {
        id,
        title: task.title.clone(),
        content: task.content.clone(),
        updated_at: Some(get_e8_time()),
        user_id: task.user_id,
    };
    let (code, message, task) = task_controller::put_task_by_id_controller(id, &put_task.into());
    println!("{task:?}");
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":task
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    params(("id", description = "delete id"),),
    responses(
        (status = 200, description = "delete successfully", body = Task),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[delete("/task/<id>")]
pub fn delete_task_by_id(id: i32) -> Json<serde_json::Value> {
    let (code, message, deleted_task) = task_controller::delete_task_by_id_controller(id);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "date":deleted_task
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "found successfully", body = Vec<Task>),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[get("/task")]
pub fn get_all_tasks() -> Json<serde_json::Value> {
    let (code, message, tasks) = task_controller::get_all_tasks_controller();
    let response = json!({
        "code":code,
        "message": message,
        "data":tasks
    });
    Json(serde_json::from_value(response).unwrap())
}

#[utoipa::path(
    responses(
        (status = 200, description = "created successfully", body = Task),
        (status = NOT_FOUND, description = "err") 
    )
)]
#[post("/task", data = "<task>")]
pub fn insert_single_task(task: Json<NewTask>) -> Json<serde_json::Value> {
    let mut raw_task: NewTask = task.into_inner();
    let (code, message, result_task): (i32, String, Task) =
        task_controller::insert_single_task_controller(&mut raw_task);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":result_task
    }))
    .unwrap();
    Json(response)
}

#[utoipa::path(
    responses(
        (status = 200, description = "found successfully", body = Vec<Task>),
        (status = NOT_FOUND, description = "not found") 
    )
)]
#[post("/task/filter", data = "<params>")]
pub fn get_tasks_by_params(
    params: Json<crate::routes::models::task_param::TaskParam>,
) -> Json<serde_json::Value> {
    let (code, message, filtered_tasks) = task_controller::get_tasks_by_params_controller(&params);
    let response = serde_json::from_value(json!({
        "code":code,
        "message":message,
        "data":filtered_tasks,

    }))
    .unwrap();
    Json(response)
}

#[get("/")]
pub fn index() -> &'static str { "hello world!" }

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
pub fn options_task_filter() -> Status { Status::Ok }

use crate::controllers::task_controller;
use crate::models::task::{NewTask, PatchTask, Task};
use crate::services::task_service::GetTask;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, post, put};

/// # Note
/// 若业务逻辑复杂则启用controller层
/// 目前只是把业务逻辑简单包含在路由中
/// ## Put和Patch
/// https://ihower.tw/blog/archives/6483
/// PUT 比較正確的定義是 Replace (Create or Update)，
/// 例如PUT/items/1的意思是替換/items/1，如果已經存在就替換，沒有就新增。
/// PUT必須包含items/1的所有屬性資料。

#[get("/task/<id>")]
pub fn get_task_by_id(id: i32) -> Json<Task> {
    let task = Task::get_task_by_id(id);
    Json(task)
}

#[patch("/task/<id>", data = "<task>")]
pub fn update_task_by_id(id: i32, task: Json<PatchTask>) -> Json<Task> {
    let updated_task = Task::update_task_by_id(id, task.into_inner());
    Json(updated_task)
}
#[put("/task", data = "<task>")]
pub fn put_task(task: Json<Task>) -> Json<Task> {
    let mut raw_task: Task = task.clone().into_inner();
    let task = task_controller::put_task(&mut raw_task);
    Json(task)
}

#[delete("/task/<id>")]
pub fn delete_task_by_id(id: i32) -> Json<Task> {
    let deleted_task = Task::delete_task_by_id(id);
    Json(deleted_task)
}

#[get("/task")]
pub fn get_all_tasks() -> Json<Vec<Task>> {
    let tasks = Task::get_all_tasks();
    Json(tasks)
}

#[post("/task", data = "<task>")]
pub fn insert_single_task(task: Json<NewTask>) -> Json<Task> {
    let mut raw_task: NewTask = task.into_inner();
    let result_task: Task = task_controller::insert_single_task_controller(&mut raw_task);
    println!("{result_task:?}");
    Json(result_task)
}

#[get("/")]
pub fn index() -> &'static str { "hello world!" }

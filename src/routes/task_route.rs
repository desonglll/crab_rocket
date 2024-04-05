use crate::controllers::task_controller;
use crate::models::task::{NewTask, PutTask, Task};
use crate::services::task_service::GetTask;
use rocket::serde::json::Json;
use rocket::{get, post, put};

/// # Note
/// 若业务逻辑复杂则启用controller层
/// 目前只是把业务逻辑简单包含在路由中

#[get("/task/<id>")]
pub fn get_task_by_id(id: i32) -> Json<Task> {
    let task = Task::get_task_by_id(id);
    Json(task)
}

#[put("/task/<id>", data = "<task>")]
pub fn update_task_by_id(id: i32, task: Json<PutTask>) -> Json<Task> {
    let updated_task = Task::update_task_by_id(id, task.into_inner());
    Json(updated_task)
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

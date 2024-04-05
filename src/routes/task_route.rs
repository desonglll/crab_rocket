use crate::models::task::{NewTask, PutTask, Task};
use crate::services::task_service::GetTask;
use crate::utils::time::get_e8_time;
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
    // let raw_task = task.clone().into_inner();
    let updated_task = Task::update_task_by_id(id, task.into_inner());
    Json(updated_task)
}

#[get("/task")]
pub fn get_all_tasks() -> Json<Vec<Task>> {
    let tasks = Task::get_all_tasks();
    Json(tasks)
}

#[post("/task", data = "<task>")]
pub fn insert_single_task(task: Json<NewTask>) {
    let mut raw_task: NewTask = task.clone().into_inner();
    // TODO: Move to controller in the future.
    // Handle None date time
    if raw_task.created_at == None {
        raw_task.created_at = Some(get_e8_time());
    }
    if raw_task.updated_at == None {
        raw_task.updated_at = Some(get_e8_time());
    }
    let result_task = Task::insert_single_task(raw_task.into());
    println!("{result_task:?}");
}

#[get("/")]
pub fn index() -> &'static str { "hello world!" }

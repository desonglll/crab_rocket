use crate::models::task::Task;
use crate::services::task_service::GetTask;
use rocket::serde::json::Json;
use rocket::{get, post};

/// # Note
/// 若业务逻辑复杂则启用controller层
/// 目前只是把业务逻辑简单包含在路由中

#[get("/task/<id>")]
pub fn get_task_by_id(id: u64) -> Json<Task> {
    let task = Task::get_task_by_id(id);
    match task {
        Some(t) => Json(t),
        _ => Json(Task::new(0, "not found".to_string(), "".to_string())),
    }
}

#[get("/task")]
pub fn get_all_tasks() -> Json<Vec<Task>> {
    let tasks = Task::get_all_tasks();
    match tasks {
        Some(t) => Json(t),
        _ => Json(Vec::new()), // 如果出现错误或者没有数据，返回一个空的 JSON 对象
    }
}

#[post("/task", data = "<task>")]
pub fn insert_single_task(task: Json<Task>) {
    let result_task = Task::insert_single_task(task.into_inner()).unwrap();
    println!("{result_task:?}");
}

#[get("/")]
pub fn index() -> &'static str {
    "hello world!"
}

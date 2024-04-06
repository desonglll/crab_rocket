use crate::establish_pg_connection;
use crate::mappers::task_mapper;

use crate::services::task_service::GetTask;
use crate::{
    models::task::{NewTask, Task},
    utils::time::get_e8_time,
};

pub fn get_all_tasks_controller() -> (i32, &'static str, Vec<Task>) {
    match Task::get_all_tasks() {
        Ok(all_tasks) => (200, "OK", all_tasks),
        Err(e) => {
            if e == diesel::result::Error::NotFound {
                (404, "Resource Not Found", Vec::new())
            } else {
                (404, "Not Found", Vec::new())
            }
        }
    }
}

pub fn get_task_by_id_controller(id: i32) -> (i32, &'static str, Task) {
    match Task::get_task_by_id(id) {
        Ok(task) => (200, "OK", task),
        Err(e) => {
            if e == diesel::result::Error::NotFound {
                (404, "Resource Not Found", Task::new_empty())
            } else {
                (404, "Not Found", Task::new_empty())
            }
        }
    }
}

pub fn insert_single_task_controller(raw_task: &mut NewTask) -> (i32, &'static str, Task) {
    // TODO: Move to controller in the future.
    // Handle None date time
    if raw_task.created_at == None {
        raw_task.created_at = Some(get_e8_time());
    }
    if raw_task.updated_at == None {
        raw_task.updated_at = Some(get_e8_time());
    }
    match Task::insert_single_task(raw_task.clone()) {
        Ok(result_task) => (201, "Created", result_task),
        Err(_) => (204, "CREATED ERROR", Task::new_empty()),
    }
}

pub fn put_task_by_id_controller(
    id: i32,
    task: crate::models::task::PutTask,
) -> (i32, &'static str, Task) {
    let mut conn = establish_pg_connection();
    if task_mapper::check_exist_task_by_id(&mut conn, id) {
        // Update all fields if exists.
        println!("Update all fields.");
        match Task::update_task_by_id(id, task.clone().into()) {
            Ok(task) => (200, "PUT OK", task),
            Err(_) => (204, "PUT ERROR", Task::new_empty()),
        }
    } else {
        // Insert a new task if not exists.
        println!("Insert a new task.");
        match Task::insert_full_single_task(task.clone().into()) {
            Ok(inserted_new_task) => (200, "PUT -> INSERT NEW OK", inserted_new_task),
            Err(_) => (204, "PUT -> INSERT NEW ERROR", Task::new_empty()),
        }
    }
}
pub fn update_task_by_id_controller(
    id: i32,
    patch_task: crate::models::task::PatchTask,
) -> (i32, &'static str, Task) {
    match Task::update_task_by_id(id, patch_task) {
        Ok(patched_task) => (200, "PATCH OK", patched_task),
        Err(_) => (204, "PATCH ERROR", Task::new_empty()),
    }
}

pub fn delete_task_by_id_controller(id: i32) -> (i32, &'static str, Task) {
    match Task::delete_task_by_id(id) {
        Ok(deleted_task) => (204, "Deleted", deleted_task),
        Err(_) => (204, "Delete Failed", Task::new_empty()),
    }
}

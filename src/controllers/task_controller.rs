use crate::establish_pg_connection;
use crate::mappers::task_mapper;

use crate::routes::models::task_param::TaskParam;
use crate::services::task_service::GetTask;
use crate::{
    models::task::{NewTask, Task},
    utils::time::get_e8_time,
};

// GOOD:
pub fn get_all_tasks_controller() -> (i32, String, Vec<Task>) {
    match Task::get_all_tasks() {
        Ok(all_tasks) => (200, String::from("OK"), all_tasks),
        Err(e) => {
            println!("{e:?}");
            (200, e.to_string(), Vec::new())
        }
    }
}

// GOOD:
pub fn get_task_by_id_controller(id: i32) -> (i32, String, Task) {
    match Task::get_task_by_id(id) {
        Ok(task) => (200, String::from("OK"), task),
        Err(e) => {
            println!("{e:?}");
            (200, e.to_string(), Task::new_empty())
        }
    }
}

pub fn insert_single_task_controller(raw_task: &mut NewTask) -> (i32, String, Task) {
    // Handle None date time
    if raw_task.created_at == None {
        raw_task.created_at = Some(get_e8_time());
    }
    if raw_task.updated_at == None {
        raw_task.updated_at = Some(get_e8_time());
    }
    match Task::insert_single_task(&raw_task.clone()) {
        Ok(result_task) => (201, String::from("Created"), result_task),
        Err(e) => (204, e.to_string(), Task::new_empty()),
    }
}

pub fn put_task_by_id_controller(
    id: i32,
    task: &crate::models::task::PutTask,
) -> (i32, String, Task) {
    match establish_pg_connection() {
        Ok(mut conn) => {
            if task_mapper::check_exist_task_by_id(&mut conn, id) {
                // Update all fields if exists.
                println!("Update all fields.");
                match Task::update_task_by_id(id, &task.clone().into()) {
                    Ok(task) => (200, String::from("PUT OK"), task),
                    Err(e) => (204, e.to_string(), Task::new_empty()),
                }
            } else {
                // Insert a new task if not exists.
                println!("Insert a new task.");
                match Task::insert_full_single_task(&task.clone().into()) {
                    Ok(inserted_new_task) => {
                        (200, String::from("PUT -> INSERT NEW OK"), inserted_new_task)
                    }
                    Err(e) => (204, e.to_string(), Task::new_empty()),
                }
            }
        }
        Err(e) => (504, e.to_string(), Task::new_empty()),
    }
}
pub fn update_task_by_id_controller(
    id: i32,
    patch_task: &crate::models::task::PatchTask,
) -> (i32, String, Task) {
    match Task::update_task_by_id(id, patch_task) {
        Ok(patched_task) => (200, String::from("PATCH OK"), patched_task),
        Err(e) => (204, e.to_string(), Task::new_empty()),
    }
}

pub fn delete_task_by_id_controller(id: i32) -> (i32, String, Task) {
    match Task::delete_task_by_id(id) {
        Ok(deleted_task) => (204, String::from("Deleted"), deleted_task),
        Err(e) => (204, e.to_string(), Task::new_empty()),
    }
}
pub fn get_tasks_by_params_controller(params: &TaskParam) -> (i32, String, Vec<Task>) {
    match Task::filter_tasks_by_params(params) {
        Ok(filtered_tasks) => (200, String::from("GET TASKS BY PARAMS OK"), filtered_tasks),
        Err(e) => (204, e.to_string(), Vec::new()),
    }
}

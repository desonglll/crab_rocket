use crate::establish_pg_connection;
use crate::mappers::task_mapper;

use crate::services::task_service::GetTask;
use crate::{
    models::task::{NewTask, Task},
    utils::time::get_e8_time,
};

// GOOD:
pub fn get_all_tasks_controller() -> (i32, &'static str, Vec<Task>) {
    match Task::get_all_tasks() {
        Ok(all_tasks) => (200, "OK", all_tasks),
        Err(e) => {
            if let Some(diesel_error) = e.downcast_ref::<diesel::result::Error>() {
                if diesel_error == &diesel::result::Error::NotFound {
                    println!("{diesel_error:?}");
                    (404, "Resource Not Found", Vec::new())
                } else {
                    println!("{diesel_error:?}");
                    (404, "Not Found", Vec::new())
                }
            } else {
                // 如果无法转换为 diesel::result::Error，则进行其他处理
                println!("{e:?}");
                (404, "Not Found", Vec::new())
            }
        }
    }
}

// GOOD:
pub fn get_task_by_id_controller(id: i32) -> (i32, &'static str, Task) {
    match Task::get_task_by_id(id) {
        Ok(task) => (200, "OK", task),
        Err(e) => {
            if let Some(diesel_error) = e.downcast_ref::<diesel::result::Error>() {
                if diesel_error == &diesel::result::Error::NotFound {
                    println!("{diesel_error:?}");
                    (404, "Resource Not Found", Task::new_empty())
                } else {
                    println!("{diesel_error:?}");
                    (404, "Not Found", Task::new_empty())
                }
            } else {
                // 如果无法转换为 diesel::result::Error，则进行其他处理
                println!("{e:?}");
                (404, "Not Found", Task::new_empty())
            }
        }
    }
}

pub fn insert_single_task_controller(raw_task: &mut NewTask) -> (i32, &'static str, Task) {
    // Handle None date time
    if raw_task.created_at == None {
        raw_task.created_at = Some(get_e8_time());
    }
    if raw_task.updated_at == None {
        raw_task.updated_at = Some(get_e8_time());
    }
    match Task::insert_single_task(&raw_task.clone()) {
        Ok(result_task) => (201, "Created", result_task),
        Err(_) => (204, "CREATED ERROR", Task::new_empty()),
    }
}

pub fn put_task_by_id_controller(
    id: i32,
    task: &crate::models::task::PutTask,
) -> (i32, &'static str, Task) {
    match establish_pg_connection() {
        Ok(mut conn) => {
            if task_mapper::check_exist_task_by_id(&mut conn, id) {
                // Update all fields if exists.
                println!("Update all fields.");
                match Task::update_task_by_id(id, &task.clone().into()) {
                    Ok(task) => (200, "PUT OK", task),
                    Err(_) => (204, "PUT ERROR", Task::new_empty()),
                }
            } else {
                // Insert a new task if not exists.
                println!("Insert a new task.");
                match Task::insert_full_single_task(&task.clone().into()) {
                    Ok(inserted_new_task) => (200, "PUT -> INSERT NEW OK", inserted_new_task),
                    Err(_) => (204, "PUT -> INSERT NEW ERROR", Task::new_empty()),
                }
            }
        }
        Err(_) => (504, "establish_pg_connection error", Task::new_empty()),
    }
}
pub fn update_task_by_id_controller(
    id: i32,
    patch_task: &crate::models::task::PatchTask,
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

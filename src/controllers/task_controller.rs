use crate::establish_pg_connection;
use crate::mappers::task_mapper;

use crate::services::task_service::GetTask;
use crate::{
    models::task::{NewTask, Task},
    utils::time::get_e8_time,
};

pub fn insert_single_task_controller(raw_task: &mut NewTask) -> Task {
    // TODO: Move to controller in the future.
    // Handle None date time
    if raw_task.created_at == None {
        raw_task.created_at = Some(get_e8_time());
    }
    if raw_task.updated_at == None {
        raw_task.updated_at = Some(get_e8_time());
    }
    let result_task = Task::insert_single_task(raw_task.clone());
    result_task
}

pub fn put_task(raw_task: &mut Task) -> Task {
    let t_id = raw_task.id;
    let mut conn = establish_pg_connection();
    if task_mapper::check_exist_task_by_id(&mut conn, t_id) {
        // Update all fields if exists.
        Task::update_task_by_id(t_id, raw_task.clone().into())
    } else {
        // Insert a new task if not exists.
        Task::insert_full_single_task(raw_task.clone())
    }
}

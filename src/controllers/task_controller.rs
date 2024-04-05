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
